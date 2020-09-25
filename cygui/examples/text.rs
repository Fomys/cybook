use cygui::{widget, Window};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use utils::Color;

#[derive(Clone)]
enum Event {}

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut w = Window::<Event>::new()?;

    let font = include_bytes!("../../Roboto-Regular.ttf") as &[u8];
    let font = Arc::new(fontdue::Font::from_bytes(
        font,
        fontdue::FontSettings {
            scale: 100.0,
            ..fontdue::FontSettings::default()
        },
    )?);
    let (metrics, bitmap) = font.rasterize('C', 100.0);

    // Output
    let mut o = File::create("fontdue.pgm").unwrap();
    let _ = o.write(format!("P5\n{} {}\n255\n", metrics.width, metrics.height).as_bytes());
    let _ = o.write(&bitmap);

    w.frame.add_widget(Box::new(widget::Text::new(
        "Hello World 😀 !".into(),
        (50, 50).into(),
        100.0,
        font.clone(),
        Color::BLACK,
        Color::WHITE,
    )));

    w.mainloop()?;
    Ok(())
}
