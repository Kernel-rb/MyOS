#![no_std]  // to disable the standard library
#![no_main] // to disable entry point of the program
#![feature(custom_test_frameworks)]
#![test_runner(my_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
// #![feature(asm)] it's already enabled by default

// mod vga_buffer; // import the module vga_buffer
// mod serial; // import the module serial
use core::panic::PanicInfo;
use my_os::{gdt::init, println}; // to import the println macro from the my_os crate




// -------------------------------------- Entry Point  --------------------------------------
#[no_mangle] // mt encodich lia had l function name
pub extern "C" fn _start() -> ! {
    // the default entry point of the program && also is the entry point of every OS
    // "C" => to tell the compiler to use the C calling convention
    // "!": the function never returns 
    
    // welcome message
    println!("Welcome to  RustOS , version: {}", "0.1.0");
    my_os::init(); // to initialize the OS

    
    // to test the breakpoint exception
    #[cfg(test)]
    test_main(); 

    // to test the breakpoint exception
    println!("It did not crash!");



    loop {}
}



// -------------------------------------- Panic Handler  --------------------------------------
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
} 

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    my_os::test_panic_handler(info)
}



