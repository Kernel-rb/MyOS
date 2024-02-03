#![no_std]  // to disable the standard library
#![no_main] // to disable entry point of the program

use core::panic::PanicInfo;

#[no_mangle] // mt encodich lia had l function name
pub extern "C" fn _start() -> ! {
    // the default entry point of the program && also is the entry point of every OS
    // "C" => to tell the compiler to use the C calling convention
    // "!": the function never returns 
    loop {}
}


// #[cfg(not(test))]
#[panic_handler] // when the program panics, it will call this function
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}



