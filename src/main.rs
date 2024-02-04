#![no_std]  // to disable the standard library
#![no_main] // to disable entry point of the program
// #![feature(asm)] it's already enabled by default

mod vga_buffer; // import the module vga_buffer


use core::panic::PanicInfo;
#[panic_handler] // when the program panics, it will call this function
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}



#[no_mangle] // mt encodich lia had l function name
pub extern "C" fn _start() -> ! {
    // the default entry point of the program && also is the entry point of every OS
    // "C" => to tell the compiler to use the C calling convention
    // "!": the function never returns 
    print!("Welcome to RustOS , version : {}", 0.1);

    loop {}
}



