use pic8259::ChainedPics;
use x86_64::structures::idt::InterruptStackFrame;
pub mod keyboard;

pub const PIC1_OFFSET: u8 = 32;
pub const PIC2_OFFSET: u8 = PIC1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC1_OFFSET, PIC2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

use crate::print;
pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    print!(".");
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn test_timer_interrupt_index_as_u8() {
        assert_eq!(InterruptIndex::Timer.as_u8(), 32);
    }

    #[test_case]
    fn test_keyboard_interrupt_index_as_u8() {
        assert_eq!(InterruptIndex::Keyboard.as_u8(), 33);
    }

    #[test_case]
    fn test_timer_interrupt_index_as_usize() {
        assert_eq!(InterruptIndex::Timer.as_usize(), 32);
    }

    #[test_case]
    fn test_keyboard_interrupt_index_as_usize() {
        assert_eq!(InterruptIndex::Keyboard.as_usize(), 33);
    }
}
