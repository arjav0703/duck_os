#![no_std]
#![no_main]

mod vga;
use vga::Writer;

use core::fmt::Write;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::default());
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut w = WRITER.lock();

    write!(w, "Welcome to DuckOS!\n");
    write!(w, "<3");

    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
