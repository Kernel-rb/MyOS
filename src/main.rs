#![no_std]  // to disable the standard library
#![no_main] // to disable entry point of the program
#![feature(custom_test_frameworks)]
#![test_runner(my_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
// #![feature(asm)] it's already enabled by default

// mod vga_buffer; // import the module vga_buffer
// mod serial; // import the module serial
use core::panic::PanicInfo;
use my_os::println; // to import the println macro from the my_os crate



// -------------------------------------- Entry Point  --------------------------------------
#[no_mangle] // mt encodich lia had l function name
pub extern "C" fn _start() -> ! {
    // the default entry point of the program && also is the entry point of every OS
    // "C" => to tell the compiler to use the C calling convention
    // "!": the function never returns 
    println!("Welcome to  RustOS , version: {}", "0.1.0");
    #[cfg(test)]
    test_main();   
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






// -------------------------------------- Test Runner  --------------------------------------
// #[test_case]
// fn trivial_assertion(){ // the test function should return nothing
//     assert_eq!(1, 1); // if the condition is true, it will do nothing, otherwise it will panic
// }



// // --------------------------------------- Exit Qemu : ---------------------------------------
// #[derive(Debug, Clone, Copy, PartialEq, Eq)] // to derive some traits for the enum ( enum est un type de donnée qui peut avoir plusieurs valeurs )
// #[repr(u32)] // to represent the enum as a 32-bit integer
// pub enum QemuExitCode {
//     Success = 0x10, // to represent the success
//     Failed = 0x11, // to represent the failure
// }
// pub fn exit_qemu(exit_code: QemuExitCode) {
//     use x86_64::instructions::port::Port; // to use the Port struct from the x86_64 crate
//     unsafe{ // we use unsafe because we are doing some low-level operations and to tell the compiler that we are aware of the risks
//     let mut port =Port::new(0xf4); // to create a new Port instance ( read notes to understand the port number)
//     port.write(exit_code as u32); // to write the exit code to the port
//     }
// }