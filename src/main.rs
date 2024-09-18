#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!"); 
    println!();
    println!("This is my OS created in Rust\n");
    println!("--Gu4rdI4N");
    blog_os::init(); 
    // fn stack_overflow() {
    //     stack_overflow();
    // }

    
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


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}