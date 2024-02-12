use uart_16550::SerialPort; // import the module SerialPort from the crate uart_16550 for the serial port ( est un crate qui fournit des types de données pour le port série )
use spin::Mutex; // import the module Mutex from the crate spin ( est un crate qui fournit des types de données pour la synchronisation )
use lazy_static::lazy_static; // import the module lazy_static from the crate lazy_static ( est un crate qui fournit des macros pour créer des variables statiques )

lazy_static! { // to create a static variable (variable static est une variable qui est partagée par toutes les instances de la structure)
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) }; // to create a new SerialPort instance
        serial_port.init(); // to initialize the serial port
        Mutex::new(serial_port) // to create a new Mutex instance
    };
}


#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        SERIAL1
        .lock()
        .write_fmt(args)
        .expect("Printing to serial failed");
    });
}


// -------------------------------------- Macro -------------------------------------- :
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}