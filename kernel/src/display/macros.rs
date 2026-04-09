use super::writer::Writer;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::default());
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::display::macros::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

use x86_64::instructions::interrupts;

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use crate::WRITER;
    use core::fmt::Write;

    // WRITER.lock().write_fmt(args).unwrap();

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}
