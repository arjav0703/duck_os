use super::vga::*;
use volatile::Volatile;

const BUFFER_HEIGHT: usize = 100;
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
