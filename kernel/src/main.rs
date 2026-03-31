#![no_std]
#![no_main]

mod vga;
use vga::Writer;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut writer = Writer::default();
    writer.write_string("Hello, Duck OS!\n");
    writer.write_string("VGA is working!");

    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
