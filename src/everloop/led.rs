/// Colors that represent a single LED.
#[derive(Debug)]
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
}

impl Copy for Rgbw {}
impl Clone for Rgbw {
    fn clone(&self) -> Rgbw {
        *self
    }
}
