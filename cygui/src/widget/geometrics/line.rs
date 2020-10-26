use crate::widget::Widget;
use crate::{Event, Handler};
use cgmath::Vector2;
use cybuf::Drawable;
use std::sync::mpsc::Sender;
use utils::Color;

pub struct Line {
    start: Vector2<isize>,
    stop: Vector2<isize>,
    color: Color,
}

impl Line {
    pub fn new(start: Vector2<isize>, stop: Vector2<isize>, color: Color) -> Self {
        Self { start, stop, color }
    }
}

impl<T> Handler<T> for Line
where
    T: Clone,
{
    fn attach(&mut self, _: Sender<Event<T>>) {}

    fn handle_event(&mut self, _: Event<T>) {}
}

impl<D, T> Widget<D, T> for Line
where
    D: Drawable,
    T: Clone,
{
    fn draw(&self, buffer: &mut D) {
        buffer.draw_line(self.start, self.stop, self.color);
    }
}
