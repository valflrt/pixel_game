#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const RED: Color = Color {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const GREEN: Color = Color {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };
    pub const BLUE: Color = Color {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const TRANSPARENT: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub fn from_bytes(bytes: [u8; 4]) -> Self {
        Color {
            r: bytes[0],
            g: bytes[1],
            b: bytes[2],
            a: bytes[3],
        }
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}
