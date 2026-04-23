#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

mod display;
use bootloader::{BootInfo, entry_point};
use display::writer::Writer;
mod exit;
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

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::default());
}

entry_point!(start);
pub fn start(boot_info: &'static BootInfo) -> ! {
    println!("Welcome to DuckOS!");
    println!("<3");

    interrupts::init_idt();
    shell::SHELL.lock().prompt();

    let l4_table = memory::fetch_l4_table(VirtAddr::new(boot_info.physical_memory_offset));
    let offset_table =
        unsafe { OffsetPageTable::new(l4_table, VirtAddr::new(boot_info.physical_memory_offset)) };

    // let addresses = [
    //     //vga buffer page
    //     0xb8000,
    //     // some code page
    //     0x201008,
    //     boot_info.physical_memory_offset,
    // ];
    //
    // for &address in &addresses {
    //     let virt_addr = VirtAddr::new(address);
    //     let phys_addr = offset_table.translate_addr(virt_addr);
    //     println!("virtual: {:#x} -> physical: {:?}", virt_addr, phys_addr);
    // }
    // for (i, entry) in l4_table.iter().enumerate() {
    //     if !entry.is_unused() {
    //         println!("L4 Entry {}: {:?}", i, entry);
    //
    //         let l3_table = memory::fetch_next_table(
    //             VirtAddr::new(boot_info.physical_memory_offset),
    //             entry.frame().unwrap(),
    //         );
    //
    //         for (j, entry) in l3_table.iter().enumerate() {
    //             if !entry.is_unused() {
    //                 println!("L3 Entry {}: {:?}", j, entry);
    //             }
    //         }
    //     }
    // }
    //
    // x86_64::instructions::interrupts::int3();

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
