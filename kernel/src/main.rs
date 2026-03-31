#![no_std]
#![no_main]

mod vga;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    render_string("meow :3", VgaAttribute::default());

    loop {}
}

use core::panic::PanicInfo;

use crate::vga::{Color, VgaAttribute, render_string};

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
