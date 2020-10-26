use cygui::{utils::font::Font, widget::text::TextBuilder, Window};
use std::{error::Error, rc::Rc};
use utils::Color;

#[derive(Clone)]
enum Event {}

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut w = Window::<Event>::new()?;

    let font = Rc::new(fontdue::Font::from_bytes(
        include_bytes!("../../Roboto-Regular.ttf") as &[u8],
        fontdue::FontSettings {
            scale: 100.0,
            ..fontdue::FontSettings::default()
        },
    )?);

    let text = TextBuilder::new(
        (50, 50).into(),
        Font {
            font,
            size: 100.0,
            fg: Color { g: 255, a: 0 },
            bg: Color { g: 0, a: 0 },
        },
    )
    .with_text("Hello World 😀 !".into())
    .build();

    w.frame.add_widget(Box::new(text));

    w.mainloop()?;
    Ok(())
}
