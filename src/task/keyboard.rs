use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use crate::println; 


// The size of the scancode queue 
static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();


// Called by the keyboard interrupt handler to add a scancode to the queue
pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            println!("WARNING: scancode queue full; dropping keyboard input");
        }
    } else {
        println!("WARNING: scancode queue uninitialized");
    }
}