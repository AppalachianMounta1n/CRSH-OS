use uart_16550::{Config, Uart16550Tty, backend::PioBackend};
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SERIAL1: Mutex<Uart16550Tty<PioBackend>> = Mutex::new(unsafe {
        Uart16550Tty::new_port(0x3F8, Config::default()).expect("Failed to initialize UART.")
    });
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Printing to serial failed.");
}

//print to hos through serial interface
#[macro_export]
macro_rules! serialPrintr {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

//print to host through serial interface and append newline
#[macro_export]
macro_rules! serialLnprintr {
    () => ($crate::serialPrintr!("\n"));
    ($fmt:expr) => ($crate::serialPrintr!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serialPrintr!(concat!($fmt, "\n"), $($arg)*));
}