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

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn test_write_byte_increments_column() {
        let mut writer = Writer::default();
        writer.write_byte(b'A');
        assert_eq!(writer.column_position, 1);
    }

    #[test_case]
    fn test_newline_resets_column_and_increments_row() {
        let mut writer = Writer::default();
        writer.write_byte(b'A');
        writer.write_byte(b'\n');
        assert_eq!(writer.column_position, 0);
        assert_eq!(writer.row_position, 1);
    }

    #[test_case]
    fn test_write_string_increments_column() {
        let mut writer = Writer::default();
        writer.write_string("hello");
        assert_eq!(writer.column_position, 5);
    }

    #[test_case]
    fn test_write_string_with_newline_advances_row() {
        let mut writer = Writer::default();
        writer.write_string("hi\nthere");
        assert_eq!(writer.row_position, 1);
        assert_eq!(writer.column_position, 5);
    }

    #[test_case]
    fn test_clear_screen_resets_position() {
        let mut writer = Writer::default();
        writer.write_string("hello");
        writer.write_byte(b'\n');
        writer.clear_screen();
        assert_eq!(writer.column_position, 0);
        assert_eq!(writer.row_position, 0);
    }

    #[test_case]
    fn test_write_byte_wraps_at_buffer_width() {
        let mut writer = Writer::default();
        for _ in 0..BUFFER_WIDTH {
            writer.write_byte(b'x');
        }
        assert_eq!(writer.column_position, BUFFER_WIDTH);
        writer.write_byte(b'y');
        // new_line was triggered: column reset to 0, row advanced, then 'y' written
        assert_eq!(writer.column_position, 1);
        assert_eq!(writer.row_position, 1);
    }

    #[test_case]
    fn test_write_byte_stores_correct_character() {
        let mut writer = Writer::default();
        writer.write_byte(b'X');
        let written = writer.buffer.chars[0][0].read();
        assert_eq!(written.ascii_character, b'X');
    }

    #[test_case]
    fn test_clear_screen_fills_with_spaces() {
        let mut writer = Writer::default();
        writer.write_byte(b'Z');
        writer.clear_screen();
        let written = writer.buffer.chars[0][0].read();
        assert_eq!(written.ascii_character, b' ');
    }

    #[test_case]
    fn test_write_byte_stores_correct_color_attribute() {
        let mut writer = Writer::default();
        writer.write_byte(b'A');
        let written = writer.buffer.chars[0][0].read();
        assert_eq!(written.attributes, writer.color_code.to_byte());
    }
}
