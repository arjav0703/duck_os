#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga;
use vga::Writer;
mod exit;
mod serial_port;
use exit::{QemuExitCode, exit_qemu};

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::default());
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Welcome to DuckOS!");
    println!("<3");

    #[cfg(test)]
    test_main();

    // panic!("The duck is dead :(");

    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panic: {}", info);
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn assertion() {
    serial_println!("assertion..");
    let one = 1;
    assert_eq!(1, one);
    serial_println!("assertion [ok]");
}
