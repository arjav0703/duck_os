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
    pub fn new(foreground: Color, background: Color, blink: bool) -> Self {
        Self {
            foreground,
            background,
            blink,
        }
    }

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

pub fn render_string(str: &str, attr: VgaAttribute) {
    let buf = 0xb8000 as *mut u8;
    let s = str.as_bytes();
    let attr_byte = attr.to_byte();

    for (i, &byte) in s.iter().enumerate() {
        unsafe {
            *buf.add(i * 2) = byte; // Character
            *buf.add(i * 2 + 1) = attr_byte; // Attribute with blinking support
        }
    }
}
