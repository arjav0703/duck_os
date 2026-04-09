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
pub struct ScreenChar {
    pub ascii_character: u8,
    pub attributes: u8,
}
