use crate::WRITER;
use core::fmt;

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
    use core::fmt::Write;

    // WRITER.lock().write_fmt(args).unwrap();

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}
