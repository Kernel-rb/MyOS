
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame}; //  to import the InterruptDescriptorTable and InterruptStackFrame types for setting up the IDT
use lazy_static::lazy_static; // to import the lazy_static macro
use pic8259::ChainedPics; // POUR importer le type ChainedPics 
use spin;  // to import the Mutex type
use crate::{gdt , println , print}; // to import the gdt module



// pour initialiser les PICs
pub const PIC_1_OFFSET: u8 = 32; // to define the offset of the first PIC
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8; // to define the offset of the second PIC  le décalage )

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
//  to define the interrupt index
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET, // to define the timer interrupt
    Keyboard, // to define the keyboard interrupt
}
#[allow(dead_code)]
impl InterruptIndex {
    fn as_u8(self) -> u8 { // to convert the interrupt index to u8
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8()) // to convert the interrupt index to usize
    }
}

//  to define a function that will be called when a breakpoint exception occurs
pub fn init_idt() {
    IDT.load();
}

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
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler); 
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt.page_fault.set_handler_fn(page_fault_handler); // to set the page fault handler function
        idt
    };
}


// to define the breakpoint exception handler function
extern "x86-interrupt" fn breakpoint_handler(stack_frame:InterruptStackFrame) {
    // to print a me    ssage when the breakpoint exception occurs
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}


extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    print!(".");
    unsafe{
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8()); // to notify the end of the interrupt to the PIC 
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::instructions::port::Port;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Azerty, ScancodeSet1>> =
            Mutex::new(Keyboard::new(layouts::Azerty, ScancodeSet1,
                HandleControl::Ignore)
            );
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}


use x86_64::structures::idt::PageFaultErrorCode;
use crate::hlt_loop;

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    hlt_loop();
}


// -------------------------------------- Test  --------------------------------------
#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3(); // to trigger a breakpoint exception
}


