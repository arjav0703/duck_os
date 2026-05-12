use crate::WRITER;
use crate::{exec, fs};
use alloc::{string::String, vec::Vec};
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
                    for i in self.pos..INPUT_BUFFER_SIZE - 1 {
                        self.buffer[i] = self.buffer[i + 1];
                    }

                    self.buffer[INPUT_BUFFER_SIZE - 1] = 0;
                    let mut writer = WRITER.lock();

                    write!(writer, "\nduckos> ").unwrap();
                    for i in 0..self.pos {
                        if self.buffer[i] != 0 {
                            writer.write_char(self.buffer[i] as char).unwrap();
                        }
                    }
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
        match parse_command(input) {
            Command::Help => {
                writeln!(
                    writer,
                    "Built-in commands: help, clear, ls, cat, write, rm, exec"
                )
                .ok();
            }
            Command::Clear => {
                writer.clear_screen();
            }
            Command::Ls => {
                let entries = fs::list();
                if entries.is_empty() {
                    writeln!(writer, "(empty)").ok();
                } else {
                    for name in entries {
                        writeln!(writer, "{}", name).ok();
                    }
                }
            }
            Command::Cat { name } => match fs::read(name.as_str()) {
                Some(data) => {
                    for &b in data.iter() {
                        if b == b'\n' {
                            writeln!(writer).ok();
                        } else {
                            writer.write_char(b as char).ok();
                        }
                    }
                    writeln!(writer).ok();
                }
                None => {
                    writeln!(writer, "file not found: {}", name).ok();
                }
            },
            Command::Write { name, data } => {
                fs::write(name.as_str(), data.as_bytes());
                writeln!(writer, "ok").ok();
            }
            Command::Rm { name } => {
                if fs::delete(name.as_str()) {
                    writeln!(writer, "ok").ok();
                } else {
                    writeln!(writer, "file not found: {}", name).ok();
                }
            }
            Command::Exec { name } => match fs::read(name.as_str()) {
                Some(data) => match exec::parse_elf(&data.to_vec()) {
                    Ok(info) => {
                        writeln!(writer, "ELF64 x86_64").ok();
                        writeln!(writer, "entry: 0x{:x}", info.entry).ok();
                        writeln!(writer, "segments: {}", info.segments.len()).ok();
                        for (idx, seg) in info.segments.iter().enumerate() {
                            writeln!(
                                writer,
                                "[{}] vaddr=0x{:x} filesz={} memsz={} flags=0x{:x}",
                                idx, seg.vaddr, seg.filesz, seg.memsz, seg.flags
                            )
                            .ok();
                        }
                    }
                    Err(err) => {
                        writeln!(writer, "exec parse error: {:?}", err).ok();
                    }
                },
                None => {
                    writeln!(writer, "file not found: {}", name).ok();
                }
            },
            Command::Empty => {}
            Command::Unknown(cmd) => {
                writeln!(writer, "Unknown command: {}", cmd).ok();
            }
        }
    }
}

enum Command {
    Help,
    Clear,
    Ls,
    Cat { name: String },
    Write { name: String, data: String },
    Rm { name: String },
    Exec { name: String },
    Empty,
    Unknown(String),
}

fn parse_command(input: &str) -> Command {
    let mut parts = input.split_whitespace();
    let cmd = match parts.next() {
        Some(cmd) => cmd,
        None => return Command::Empty,
    };

    match cmd {
        "help" => Command::Help,
        "clear" => Command::Clear,
        "ls" => Command::Ls,
        "cat" => match parts.next() {
            Some(name) => Command::Cat { name: name.into() },
            None => Command::Unknown("cat".into()),
        },
        "write" => match (parts.next(), rest_as_string(parts)) {
            (Some(name), Some(data)) => Command::Write {
                name: name.into(),
                data,
            },
            _ => Command::Unknown("write".into()),
        },
        "rm" => match parts.next() {
            Some(name) => Command::Rm { name: name.into() },
            None => Command::Unknown("rm".into()),
        },
        "exec" => match parts.next() {
            Some(name) => Command::Exec { name: name.into() },
            None => Command::Unknown("exec".into()),
        },
        "" => Command::Empty,
        _ => Command::Unknown(cmd.into()),
    }
}

fn rest_as_string<'a>(mut parts: impl Iterator<Item = &'a str>) -> Option<String> {
    let mut data: Vec<&'a str> = Vec::new();
    for part in parts {
        data.push(part);
    }
    if data.is_empty() {
        return None;
    }
    let mut out = String::new();
    for (idx, part) in data.iter().enumerate() {
        if idx > 0 {
            out.push(' ');
        }
        out.push_str(part);
    }
    Some(out)
}

lazy_static::lazy_static! {
    pub static ref SHELL: Mutex<Shell> = Mutex::new(Shell::new());
}
