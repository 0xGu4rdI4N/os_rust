#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::println;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);
// #[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use blog_os::memory;
    use blog_os::memory::BootInfoFrameAllocator;
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    use x86_64::{structures::paging::Page, VirtAddr}; // new import
    println!("Hello World{}", "!"); 
    println!();
    println!("This is my OS created in Rust\n");
    println!("--Gu4rdI4N");
    blog_os::init(); 
    // fn stack_overflow() {
    //     stack_overflow();
    // }
    use x86_64::registers::control::Cr3;

    // let (level_4_page_table, _) = Cr3::read();
    // println!("Level 4 page table at: {:?}", level_4_page_table.start_address());
    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // let l4_table = unsafe { active_level_4_table(phys_mem_offset) };
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = memory::EmptyFrameAllocator;
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];
    // for &address in &addresses {
    //     let virt = VirtAddr::new(address);
    //     // new: use the `mapper.translate_addr` method
    //     let phys = mapper.translate_addr(virt);
    //     println!("{:?} -> {:?}", virt, phys);
    // }
    // for (i, entry) in l4_table.iter().enumerate() {
    //     use x86_64::structures::paging::PageTable;
    //     if !entry.is_unused() {
    //         println!("L4 Entry {}: {:?}", i, entry);
        
    //         // get the physical address from the entry and convert it
    //         let phys = entry.frame().unwrap().start_address();
    //         let virt = phys.as_u64() + boot_info.physical_memory_offset;
    //         let ptr = VirtAddr::new(virt).as_mut_ptr();
    //         let l3_table: &PageTable = unsafe { &*ptr };
        
    //         // print non-empty entries of the level 3 table
    //         for (i, entry) in l3_table.iter().enumerate() {
    //             if !entry.is_unused() {
    //                 println!("  L3 Entry {}: {:?}", i, entry);
    //             }
    //         }
    //     }
    // }
    // let p=0xcafebabe as *mut u8;
    // unsafe{*p=42;}
    // let r =0x204294 as *mut u8;
    // // unsafe{*r=12;} this will give an error since its a code page and can only be read
    // println!("so we can read from there!!");
    // let p=0x204294 as *mut u8;
    // unsafe{*p=42;}

    // stack_overflow();   
    // unsafe {
    //     *(0xcafebabe as *mut u8) = 41;
    // };
    // x86_64::instructions::interrupts::int3(); 

    #[cfg(test)]
    test_main();
    println!("It did not crash!");
    blog_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::hlt_loop();
}

