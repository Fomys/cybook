/// Color struct for e-ink screen
#[derive(Copy, Clone)]
pub struct Color {
    /// Grayscale value of current color
    pub g: u8,
    /// Transparency of current color
    pub a: u8,
}

impl Color {
    pub const TRANSPARENT: Color = Color { g: 0, a: 0 };
    pub const WHITE: Color = Color { g: 255, a: 255 };
    pub const BLACK: Color = Color { g: 0, a: 255 };
}
