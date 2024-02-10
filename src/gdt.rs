use core::ptr::addr_of;

//  GDT is a table of segment descriptors used by the processor to define the memory areas that can be accessed.
use x86_64::VirtAddr; // for virtual addresses
use x86_64::structures::tss::TaskStateSegment; // for the TaskStateSegment type to define the TSS (Task State Segment where the stack pointer is stored)
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor , SegmentSelector};
use lazy_static::lazy_static;
 

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0; // to define the index of the double fault stack in the Interrupt Stack Table (IST)

pub fn init() {
    use x86_64::instructions::tables::load_tss; // to load the TSS
    use x86_64::instructions::segmentation::{CS, Segment}; //  pour charger le segment de code

    GDT.0.load(); // to load the GDT , O => the first element of the tuple
    unsafe {
        CS::set_reg(GDT.1.code_selector); // to set the code segment register to the code selector
        load_tss(GDT.1.tss_selector); // to load the TSS
    }
}



lazy_static! {
    static ref TSS: TaskStateSegment = { 
        let mut tss = TaskStateSegment::new(); // to create a new TSS
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5 ; // (4096 * 5) bytes , 5 pages of 4KB
            static mut STACK : [u8; STACK_SIZE] = [0; STACK_SIZE]; // to create a static mutable array of 5 pages of 4KB
            let stack_start = VirtAddr::from_ptr(unsafe { addr_of!(STACK) }); // to get the virtual address of the stack 
            let stack_end = stack_start + STACK_SIZE; // to get the end of the stack
            stack_end // to return the end of the stack
        };

        tss
    };
}


lazy_static!{
    static ref GDT: (GlobalDescriptorTable , Selectors) = {
        let mut gdt  = GlobalDescriptorTable::new(); // to create a new GDT
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment()); // to add a kernel code segment to the GDT
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS)); // to add a TSS segment to the GDT
        (gdt, Selectors { code_selector, tss_selector }) // to return the GDT and the selectors
    };
}


struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}