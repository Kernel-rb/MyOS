use core::{future::Future , pin::Pin};
use alloc::boxed::Box;
use core::task::{Context , Poll};


pub mod simple_executor;




pub struct Task {
    future: Pin<Box<dyn Future<Output = ()>>
}

impl Task {
    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}