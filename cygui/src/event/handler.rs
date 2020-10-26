use crate::Event;
use std::sync::mpsc;

pub trait Handler<T: Clone> {
    fn attach(&mut self, tx: mpsc::Sender<Event<T>>);
    fn handle_event(&mut self, event: Event<T>);
}
