use crate::event::{Event, EventState};
use crate::widget::Widget;
use crate::Handler;
use cybuf::Drawable;

pub struct Frame<D>
where
    D: Drawable,
{
    handlers: Vec<Box<dyn Handler>>,
    widgets: Vec<Box<dyn Widget<D>>>,
}

impl<D> Frame<D>
where
    D: Drawable,
{
    pub fn new() -> Self {
        Self {
            handlers: vec![],
            widgets: vec![],
        }
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget<D>>) {
        self.widgets.push(widget);
    }

    pub fn add_handler(&mut self, handler: Box<dyn Handler>) {
        self.handlers.push(handler);
    }
}

impl<D> Handler for Frame<D>
where
    D: Drawable,
{
    fn handle_event(&mut self, event: &dyn Event) -> EventState {
        for handler in self.handlers.iter_mut() {
            match handler.handle_event(event) {
                EventState::Handled => return EventState::Handled,
                EventState::Pending => {}
            }
        }
        EventState::Pending
    }
}

impl<D> Widget<D> for Frame<D>
where
    D: Drawable,
{
    fn draw(&mut self, buffer: &mut D) {
        for widget in self.widgets.iter_mut() {
            widget.draw(buffer);
        }
    }
}
