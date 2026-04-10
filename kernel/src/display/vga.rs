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

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn test_vga_attribute_default_to_byte() {
        let attr = VgaAttribute::default();
        // Default: Pink (13) foreground, Black (0) background, no blink
        // bits 0-3: 13 = 0x0D, bits 4-6: 0, bit 7: 0
        assert_eq!(attr.to_byte(), 0x0D);
    }

    #[test_case]
    fn test_vga_attribute_foreground_masked_to_nibble() {
        let attr = VgaAttribute {
            foreground: Color::White, // 15 = 0x0F
            background: Color::Black,
            blink: false,
        };
        assert_eq!(attr.to_byte(), 0x0F);
    }

    #[test_case]
    fn test_vga_attribute_background_shifted_to_bits_4_to_6() {
        let attr = VgaAttribute {
            foreground: Color::Black,
            background: Color::Green, // 2 -> bits 4-6: 0x20
            blink: false,
        };
        assert_eq!(attr.to_byte(), 0x20);
    }

    #[test_case]
    fn test_vga_attribute_blink_sets_bit7() {
        let attr = VgaAttribute {
            foreground: Color::Black,
            background: Color::Black,
            blink: true,
        };
        assert_eq!(attr.to_byte(), 0x80);
    }

    #[test_case]
    fn test_vga_attribute_all_fields_combined() {
        let attr = VgaAttribute {
            foreground: Color::White, // 15 = 0x0F
            background: Color::Green, // 2 -> 0x20
            blink: true,              // 0x80
        };
        assert_eq!(attr.to_byte(), 0x0F | 0x20 | 0x80); // 0xAF
    }

    #[test_case]
    fn test_vga_attribute_background_masked_to_3_bits() {
        // White (15) background: 15 & 0x07 = 7 -> bits 4-6: 0x70
        let attr = VgaAttribute {
            foreground: Color::Black,
            background: Color::White,
            blink: false,
        };
        assert_eq!(attr.to_byte(), 0x70);
    }
}
