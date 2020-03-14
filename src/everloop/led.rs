/// Colors that represent a single LED.
#[derive(Clone, Copy, Debug)]
pub struct Rgbw {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub w: u8,
}

impl Rgbw {
    /// Shorthand way to create an RGBW instance.
    pub fn new(r: u8, g: u8, b: u8, w: u8) -> Rgbw {
        Rgbw { r, g, b, w }
    }

    /// An RGBW instance with all values at 0.
    pub fn black() -> Self {
        Self::new(0, 0, 0, 0)
    }

    /// An RGBW instance with all values maxed out.
    pub fn white() -> Self {
        Self::new(255, 255, 255, 255)
    }

    ///
    pub fn as_bytes(self) -> i32 {
        i32::from_be_bytes([self.r, self.g, self.b, self.w])
    }
}
