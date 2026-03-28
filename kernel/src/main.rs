#![no_std]
#![no_main]

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    static HELLO: &[u8] = b"Hello, world!";

    let vga_buf = 0xb8000 as *mut u8;
    render_string(HELLO, vga_buf);

    loop {}
}

fn render_string(s: &[u8], buf: *mut u8) {
    for (i, &byte) in s.iter().enumerate() {
        unsafe {
            *buf.add(i * 2) = byte;
            *buf.add(i * 2 + 1) = 0x0f; // White on black
        }
    }
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
