use crate::widget::Widget;
use crate::Handler;
use cybuf::Drawable;

pub struct Frame<D, T>
where
    D: Drawable,
{
    widgets: Vec<Box<dyn Widget<D, T>>>,
}

impl<D, T> Frame<D, T>
where
    D: Drawable,
    T: Clone,
{
    pub fn new() -> Self {
        Self { widgets: vec![] }
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget<D, T>>) {
        self.widgets.push(widget);
    }
}

impl<D, T> Handler<T> for Frame<D, T>
where
    D: Drawable,
    T: Clone,
{
    fn handle_event(&mut self, event: T) {
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
