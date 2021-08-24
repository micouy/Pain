#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn white() -> Self {
        Self {
            r: 0xff,
            g: 0xff,
            b: 0xff,
        }
    }

    pub fn black() -> Self {
        Self {
            r: 0x00,
            g: 0x00,
            b: 0x00,
        }
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self { r, g, b }
    }
}
