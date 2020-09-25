use cgmath::Vector2;
use cybuf::Drawable;
use cygui::widget::{Text, Widget};
use cygui::{Handler, Window};
use cyio::Input;
use fontdue::Font;
use std::error::Error;
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;
use utils::Color;

#[derive(Debug, Clone)]
enum Event {
    IOEvent(cyio::Event),
}

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
    pub fn new(position: Vector2<usize>, font: Rc<Font>) -> Self {
        Self {
            // En interne c'est juste du texte
            text: Text::new("".into(), position, 100.0, font, Color::BLACK, Color::WHITE),
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

impl<'f, D> Handler<Event> for EventWidget<D, Event>
where
    D: Drawable,
{
    fn handle_event(&mut self, event: Event) {
        println!("{:?}", event);
        self.text.update_text(format!("{:?}", event));
    }
}

struct IOProducer {
    input: Input,
}

impl IOProducer {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            input: Input::new()?,
        })
    }

    fn run(&mut self, tx: mpsc::Sender<Event>) -> ! {
        loop {
            match self.input.get_event() {
                Ok(event) => {
                    tx.send(Event::IOEvent(event));
                }
                _ => {}
            }
        }
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
    // Ajout d'un producteur d'évènement
    let tx = w.tx.clone();
    let mut io_producer = IOProducer::new()?;
    thread::spawn(move || io_producer.run(tx));

    let event_widget = Box::new(EventWidget::new((50, 50).into(), font.clone()));
    // Ajout des widgets
    w.frame.add_widget(event_widget);

    // Bloquant à l'infini
    w.mainloop()?;
    Ok(())
}
