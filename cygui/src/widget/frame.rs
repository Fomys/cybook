use crate::widget::Widget;
use crate::{Event, Handler};
use cybuf::Drawable;
use std::sync::mpsc;
use std::sync::mpsc::Sender;

pub struct Frame<D, T>
where
    D: Drawable,
{
    widgets: Vec<Box<dyn Widget<D, T>>>,
    tx: Option<mpsc::Sender<Event<T>>>,
}

impl<D, T> Frame<D, T>
where
    D: Drawable,
    T: Clone,
{
    pub fn new() -> Self {
        Self {
            widgets: vec![],
            tx: None,
        }
    }

    pub fn add_widget(&mut self, mut widget: Box<dyn Widget<D, T>>) {
        widget.attach(self.tx.clone().unwrap());
        self.widgets.push(widget);
    }
}

impl<D, T> Handler<T> for Frame<D, T>
where
    D: Drawable,
    T: Clone,
{
    fn attach(&mut self, tx: Sender<Event<T>>) {
        self.tx = Some(tx);
    }

    fn handle_event(&mut self, event: Event<T>) {
        for handler in self.widgets.iter_mut() {
            handler.handle_event(event.clone())
        }
    }
}

impl<D, T> Widget<D, T> for Frame<D, T>
where
    T: Clone,
    D: Drawable,
{
    fn draw(&self, buffer: &mut D) {
        for widget in self.widgets.iter() {
            widget.draw(buffer);
        }
    }
}
