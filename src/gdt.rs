//  GDT is a table of segment descriptors used by the processor to define the memory areas that can be accessed.
use x86_64::VirtAddr; // for virtual addresses
use x86_64::structures::tss::TaskStateSegment; // for the TaskStateSegment type to define the TSS (Task State Segment where the stack pointer is stored)
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor};
use lazy_static::lazy_static; 

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0; // to define the index of the double fault stack in the Interrupt Stack Table (IST)

pub fn init() {
    GDT.load(); // to load the GDT
}


lazy_static! {
    static ref TSS: TaskStateSegment = { 
        let mut tss = TaskStateSegment::new(); // to create a new TSS
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5 ; // (4096 * 5) bytes , 5 pages of 4KB
            static mut STACK : [u8; STACK_SIZE] = [0; STACK_SIZE]; // to create a static mutable array of 5 pages of 4KB
            let stack_start = VirtAddr::from_ptr(unsafe { &STACK }); // to get the virtual address of the stack 
            let stack_end = stack_start + STACK_SIZE; // to get the end of the stack
            stack_end // to return the end of the stack
        };

        tss
    };
}


lazy_static!{
    static ref GDT: GlobalDescriptorTable = {
        let mut gdt  = GlobalDescriptorTable::new(); // to create a new GDT
        gdt.add_entry(Descriptor::kernel_code_segment()); // to add a kernel code segment 
        gdt.add_entry(Descriptor::tss_segment(&TSS)); // to add a TSS segment for the GDT  
        gdt 
    };
}