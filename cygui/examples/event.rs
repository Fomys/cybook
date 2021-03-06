use cgmath::Vector2;
use cybuf::Drawable;
use cygui::{
    utils::font::Font,
    widget::{
        text::{Text, TextBuilder},
        Widget,
    },
    Handler, Window,
};
use std::sync::mpsc::Sender;
use std::{error::Error, rc::Rc};
use utils::Color;

#[derive(Debug, Clone)]
enum Event {}

// Création d'un widget particulier, qui va permettre l'affichage des events
pub struct EventWidget<D, T>
where
    D: Drawable,
    T: Clone,
{
    text: Text<D, T>,
}

impl<D, T> EventWidget<D, T>
where
    D: Drawable,
    T: Clone,
{
    pub fn new(position: Vector2<isize>, font: Font) -> Self {
        let text = TextBuilder::new(position, font).build();
        Self {
            // En interne c'est juste du texte
            text,
        }
    }
}

impl<D> Widget<D, Event> for EventWidget<D, Event>
where
    D: Drawable,
{
    fn draw(&self, buffer: &mut D) {
        self.text.draw(buffer)
    }
}

impl<D> Handler<Event> for EventWidget<D, Event>
where
    D: Drawable,
{
    fn attach(&mut self, _: Sender<cygui::Event<Event>>) {}

    fn handle_event(&mut self, event: cygui::Event<Event>) {
        println!("{:?}", event);
        self.text.update_text(format!("Event: {:?}", event));
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    // Chargement d'une fonte
    let font = include_bytes!("../../Roboto-Regular.ttf") as &[u8];
    let font = Rc::new(fontdue::Font::from_bytes(
        font,
        fontdue::FontSettings {
            scale: 100.0,
            ..fontdue::FontSettings::default()
        },
    )?);

    let mut w = Window::<Event>::new()?;
    let event_widget = Box::new(EventWidget::new(
        (50, 50).into(),
        Font {
            font,
            size: 50.0,
            fg: Color { g: 255, a: 0 },
            bg: Color { g: 0, a: 0 },
        },
    ));

    // Ajout des widgets
    w.frame.add_widget(event_widget);

    // Bloquant à l'infini
    w.mainloop()?;
    Ok(())
}
