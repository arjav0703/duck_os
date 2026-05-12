#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{BootInfo, entry_point};
use x86_64::VirtAddr;

use kernel::*;

use crate::memory::BootInfoFrameAllocator;

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
