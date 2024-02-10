
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame}; //  to import the InterruptDescriptorTable and InterruptStackFrame types for setting up the IDT
use lazy_static::lazy_static; // to import the lazy_static macro
use pic8259::ChainedPics; // POUR importer le type ChainedPics 
use spin;  // to import the Mutex type

use crate::println; // to import the println macro
use crate::gdt; // to import the gdt module

//  to define a function that will be called when a breakpoint exception occurs
pub fn init_idt() {
    IDT.load();
}

// pour initialiser les PICs
pub const PIC_1_OFFSET: u8 = 32; // to define the offset of the first PIC
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8; // to define the offset of the second PIC ( offset est le décalage )


// pour initialiser les PICs qui sont des périphériques d'interruption programmable
pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });


// the lazy macro does use unsafe behind the scenes.
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler); // to set the breakpoint handler function
        unsafe{
            idt.double_fault.set_handler_fn(double_fault_handler) // to set the double fault handler function
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX); // to set the stack index
        }
        idt
    };
}


// to define the breakpoint exception handler function
extern "x86-interrupt" fn breakpoint_handler(stack_frame:InterruptStackFrame) {
    // to print a message when the breakpoint exception occurs
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}


extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

// -------------------------------------- Test  --------------------------------------
#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3(); // to trigger a breakpoint exception
}


