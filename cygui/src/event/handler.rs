use crate::event::EventState;
use crate::Event;

pub trait Handler {
    fn handle_event(&mut self, event: &dyn Event) -> EventState;
}
