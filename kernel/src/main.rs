#![no_std]
#![no_main]

mod vga;
use vga::Writer;

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::default());
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Welcome to DuckOS!\n");
    println!("<3");

    panic!("The duck is dead :(");
    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panic: {}", info);
    loop {}
}
