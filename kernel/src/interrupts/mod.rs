mod pic;
use pic::{InterruptIndex, PICS, keyboard};
mod faults;

use x86_64::structures::idt::InterruptDescriptorTable;

use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(faults::breakpoint_handler);
        idt.double_fault
            .set_handler_fn(faults::double_fault_handler);
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(pic::timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(keyboard::keyboard_interrupt_handler);
        idt.page_fault.set_handler_fn(faults::page_fault_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

#[test_case]
fn test_interrupt() {
    x86_64::instructions::interrupts::int3();
}
