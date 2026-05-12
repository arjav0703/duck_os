#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

extern crate alloc;

mod display;
use alloc::boxed::Box;
use bootloader::{BootInfo, entry_point};
use display::writer::Writer;
mod exit;
mod exec;
mod fs;
mod memory;
mod panic;
mod serial_port;
use exit::{QemuExitCode, exit_qemu};
mod interrupts;
mod shell;

use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::{
    VirtAddr,
    structures::paging::{OffsetPageTable, Translate},
};

use crate::memory::BootInfoFrameAllocator;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::default());
}

entry_point!(start);
pub fn start(boot_info: &'static BootInfo) -> ! {
    println!("Welcome to DuckOS!");
    println!("<3");

    let mut mapper = unsafe { memory::init(VirtAddr::new(boot_info.physical_memory_offset)) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::new(&boot_info.memory_map) };

    memory::heap::init_heap(&mut mapper, &mut frame_allocator).unwrap();
    fs::init();
    interrupts::init_idt();
    shell::SHELL.lock().prompt();
    // let x = Box::new(50);
    // println!("heap value at {:p} is {}", x, x);

    #[cfg(test)]
    test_main();

    // panic!("The duck is dead :(");
    // println!("we're good :p");
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
