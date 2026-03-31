use volatile::Volatile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(dead_code)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy)]
pub struct VgaAttribute {
    pub foreground: Color,
    pub background: Color,
    pub blink: bool,
}

impl VgaAttribute {
    pub fn to_byte(self) -> u8 {
        let mut attr = (self.foreground as u8) & 0x0F; // Bits 0-3
        attr |= ((self.background as u8) & 0x07) << 4; // Bits 4-6
        if self.blink {
            attr |= 0x80; // bit 7
        }
        attr
    }
}

impl Default for VgaAttribute {
    fn default() -> Self {
        Self {
            foreground: Color::Pink,
            background: Color::Black,
            blink: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    attributes: u8,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    row_position: usize,
    color_code: VgaAttribute,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn default() -> Self {
        Self {
            column_position: 0,
            row_position: 0,
            color_code: VgaAttribute::default(),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row_position;
                let col = self.column_position;

                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    attributes: self.color_code.to_byte(),
                });

                self.column_position += 1;
            }
        }
    }

    pub fn new_line(&mut self) {
        self.column_position = 0;
        self.row_position += 1;
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }

    pub fn clear_screen(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: b' ',
                    attributes: self.color_code.to_byte(),
                });
            }
        }
        self.column_position = 0;
        self.row_position = 0;
    }
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::default());
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use crate::WRITER;
    use core::fmt::Write;

    WRITER.lock().write_fmt(args).unwrap();
}
