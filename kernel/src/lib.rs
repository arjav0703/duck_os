#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

pub fn test_runner(tests: &[&dyn Fn()]) {
    use crate::exit::{QemuExitCode, exit_qemu};

    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

use lazy_static::lazy_static;
use spin::Mutex;

use crate::display::writer::Writer;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::default());
}

extern crate alloc;

pub mod display;
pub mod exec;
pub mod exit;
pub mod fs;
pub mod interrupts;
pub mod memory;
pub mod panic;
pub mod serial_port;
pub mod shell;
