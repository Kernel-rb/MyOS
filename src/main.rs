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
use bootloader::{BootInfo,entry_point}; //BootInfo : to get the boot information from the bootloader ; entry_point : to define the entry point of the program

entry_point!(kernel_main); // to define the entry point of the program
// -------------------------------------- Entry Point  --------------------------------------
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // the default entry point of the program && also is the entry point of every OS
    // "C" => to tell the compiler to use the C calling convention
    // "!": the function never returns 
    use my_os::memory;
    use x86_64::VirtAddr;  
    // welcome message
    println!("Welcome to  RustOS , version: {}", "0.1.0");
    my_os::init(); // to initialize the OS
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut _mapper = unsafe { memory::init(phys_mem_offset) };
    let mut _frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
   

    #[cfg(test)]
    test_main();  
    println!("It did not crash!");
    my_os::hlt_loop();
}
// -------------------------------------- Panic Handler  --------------------------------------
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    my_os::hlt_loop();            // new
}
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    my_os::test_panic_handler(info)
}