#![no_std]  // to disable the standard library
#![no_main] // to disable entry point of the program
// #![feature(asm)] it's already enabled by default

use core::panic::PanicInfo;
// #[cfg(not(test))]
#[panic_handler] // when the program panics, it will call this function
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


static WELCOME: &[u8] = b"I'm Alive";// declare a static variable that contains the welcome message

#[no_mangle] // mt encodich lia had l function name
pub extern "C" fn _start() -> ! {
    // the default entry point of the program && also is the entry point of every OS
    // "C" => to tell the compiler to use the C calling convention
    // "!": the function never returns 
    let vga_buffer = 0xb8000 as *mut u8; // the address of the VGA buffer , vga buffer est un tableau de 80*25 qui contient les caractères à afficher
    for (i, &byte) in WELCOME.iter().enumerate() { // iterate over the welcome message
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; // write the byte to the VGA buffer
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // set the color of the byte
        }
    }
    loop {}
}



