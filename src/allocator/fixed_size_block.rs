struct Node{
    next:Option<&'static mut Node>,
}
const BLOCK_SIZE:&[usize]=&[8, 16, 32, 64, 128, 256, 512, 1024, 2048];

pub struct FixedAlloc{
    list_heads: [Option<&'static mut Node>; BLOCK_SIZE.len()],
    fallback_allocator: linked_list_allocator::Heap,
}   

impl FixedAlloc{
    pub const fn new() -> Self {
        const EMPTY: Option<&'static mut Node> = None;
        FixedAlloc {
            list_heads: [EMPTY; BLOCK_SIZE.len()],
            fallback_allocator: linked_list_allocator::Heap::empty(),
        }
    }
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.fallback_allocator.init(heap_start, heap_size);
    }
}

use alloc::alloc::Layout;
use core::ptr;

impl FixedAlloc {
    /// Allocates using the fallback allocator.
    fn fallback_alloc(&mut self, layout: Layout) -> *mut u8 {
        match self.fallback_allocator.allocate_first_fit(layout) {
            Ok(ptr) => ptr.as_ptr(),
            Err(_) => ptr::null_mut(),
        }
    }
}

fn list_index(layout: &Layout) -> Option<usize> {
    let required_block_size = layout.size().max(layout.align());
    BLOCK_SIZE.iter().position(|&s| s >= required_block_size)
}

use super::Locked;
use alloc::alloc::GlobalAlloc;
use core::{mem, ptr::NonNull};

unsafe impl GlobalAlloc for Locked<FixedAlloc> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut allocator = self.lock();
        match list_index(&layout) {
            Some(index) => {
                match allocator.list_heads[index].take() {
                    Some(node) => {
                        allocator.list_heads[index] = node.next.take();
                        node as *mut Node as *mut u8
                    }
                    None => {
                        // no block exists in list => allocate new block
                        let block_size = BLOCK_SIZE[index];
                        // only works if all block sizes are a power of 2
                        let block_align = block_size;
                        let layout = Layout::from_size_align(block_size, block_align)
                            .unwrap();
                        allocator.fallback_alloc(layout)
                    }
                }
            }
            None => allocator.fallback_alloc(layout),
        }    }
        
        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            let mut allocator = self.lock();
            match list_index(&layout) {
                Some(index) => {
                    let new_node = Node {
                        next: allocator.list_heads[index].take(),
                    };
                    assert!(mem::size_of::<Node>() <= BLOCK_SIZE[index]);
                    assert!(mem::align_of::<Node>() <= BLOCK_SIZE[index]);
                    let new_node_ptr = ptr as *mut Node;
                    new_node_ptr.write(new_node);
                    allocator.list_heads[index] = Some(&mut *new_node_ptr);
                }
                None => {
                    let ptr = NonNull::new(ptr).unwrap();
                    allocator.fallback_allocator.deallocate(ptr, layout);
                }
            }
        }

}

