use cygui::widget::geometrics::{Circle, Disc, Line};
use cygui::Window;
use std::error::Error;
use utils::Color;

#[derive(Clone)]
enum Event {}

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut w = Window::<Event>::new()?;

    let circle = Circle::new((300, 300).into(), 80, Color::BLACK);
    let disc = Disc::new((200, 200).into(), 100, Color::BLACK, Color::BLACK);
    let line = Line::new((500, 500).into(), (800, 100).into(), Color::BLACK);

    w.frame.add_widget(Box::new(circle));
    w.frame.add_widget(Box::new(disc));
    w.frame.add_widget(Box::new(line));
    w.mainloop()
}
