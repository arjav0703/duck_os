#![no_std]
#![no_main]

mod vga;
use vga::Writer;

use core::fmt::Write;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut writer = Writer::default();
    write!(writer, "Welcome to DuckOS!\n").unwrap();
    write!(writer, "<3").unwrap();

    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
