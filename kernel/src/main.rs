#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

mod display;
use display::writer::Writer;
mod exit;
mod panic;
mod serial_port;
use exit::{QemuExitCode, exit_qemu};
mod interrupts;

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::default());
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Welcome to DuckOS!");
    println!("<3");

    interrupts::init_idt();

    // x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    // panic!("The duck is dead :(");
    println!("we're good :p");
    loop {
        x86_64::instructions::hlt();
    }
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
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many_lines() {
    for _ in 0..50 {
        println!("test_println_many_lines output");
    }
}

#[test_case]
fn test_println_formatting() {
    println!("number: {}, string: {}, bool: {}", 42, "hello", true);
}
