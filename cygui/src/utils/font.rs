pub use fontdue;
use std::rc::Rc;
use utils::Color;

pub struct Font {
    pub font: Rc<fontdue::Font>,
    pub size: f32,
    pub fg: Color,
    pub bg: Color,
}
