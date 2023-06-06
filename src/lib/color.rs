#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const RED: Color = Color::new(255, 0, 0, 255);
    pub const GREEN: Color = Color::new(0, 255, 0, 255);
    pub const BLUE: Color = Color::new(0, 0, 255, 255);
    pub const BLACK: Color = Color::new(0, 0, 0, 255);
    pub const WHITE: Color = Color::new(255, 255, 255, 255);
    pub const TRANSPARENT: Color = Color::new(0, 0, 0, 0);

    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub const fn from_bytes(bytes: [u8; 4]) -> Self {
        Color {
            r: bytes[0],
            g: bytes[1],
            b: bytes[2],
            a: bytes[3],
        }
    }

    pub const fn to_bytes(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}
