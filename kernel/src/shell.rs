use crate::WRITER;
use core::fmt::Write;
use spin::Mutex;

const INPUT_BUFFER_SIZE: usize = 256;

pub struct Shell {
    buffer: [u8; INPUT_BUFFER_SIZE],
    pos: usize,
}

impl Shell {
    pub const fn new() -> Self {
        Self {
            buffer: [0u8; INPUT_BUFFER_SIZE],
            pos: 0,
        }
    }

    pub fn prompt(&mut self) {
        let mut writer = WRITER.lock();
        write!(writer, "duckos> ").unwrap();
    }

    pub fn input(&mut self, byte: u8) {
        match byte {
            b'\n' | b'\r' => {
                // Enter pressed
                self.execute();
                self.clear();
                self.prompt();
            }
            8 | 127 => {
                // Backspace pressed
                if self.pos > 0 {
                    self.pos -= 1;
                    let mut writer = WRITER.lock();
                    write!(writer, "\u{8} \u{8}").unwrap(); // Erase char on screen (BS SP BS)
                }
            }
            b => {
                if self.pos < INPUT_BUFFER_SIZE - 1 {
                    self.buffer[self.pos] = b;
                    self.pos += 1;
                    let mut writer = WRITER.lock();
                    writer.write_char(b as char).unwrap();
                }
            }
        }
    }

    fn clear(&mut self) {
        self.buffer = [0u8; INPUT_BUFFER_SIZE];
        self.pos = 0;
    }

    fn execute(&self) {
        let input = core::str::from_utf8(&self.buffer[..self.pos])
            .unwrap_or("")
            .trim();
        let mut writer = WRITER.lock();
        writeln!(writer, "").ok(); // new line after command entry
        match input {
            "help" => {
                writeln!(writer, "Built-in commands: help, clear").ok();
            }
            "clear" => {
                writer.clear_screen();
            }
            "" => {}
            cmd => {
                writeln!(writer, "Unknown command: {}", cmd).ok();
            }
        }
    }
}

lazy_static::lazy_static! {
    pub static ref SHELL: Mutex<Shell> = Mutex::new(Shell::new());
}
