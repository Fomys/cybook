use crate::widget::Widget;
use crate::{Event, Handler};
use cgmath::Vector2;
use cybuf::Drawable;
use std::sync::mpsc::Sender;
use utils::Color;

pub struct Disc {
    center: Vector2<isize>,
    radius: usize,
    fg: Color,
    bg: Color,
}

impl Disc {
    pub fn new(center: Vector2<isize>, radius: usize, fg: Color, bg: Color) -> Self {
        Self {
            center,
            radius,
            fg,
            bg,
        }
    }
}

impl<T> Handler<T> for Disc
where
    T: Clone,
{
    fn attach(&mut self, _: Sender<Event<T>>) {}

    fn handle_event(&mut self, _: Event<T>) {}
}

impl<D, T> Widget<D, T> for Disc
where
    D: Drawable,
    T: Clone,
{
    fn draw(&self, buffer: &mut D) {
        let mut x = 0;
        let mut y = self.radius as isize;
        let mut d = self.radius as isize - 1 as isize;
        while y >= x {
            buffer.part_vertical_line(
                self.center.x + x,
                self.center.y - y,
                self.center.y + y,
                self.bg,
            );
            buffer.part_vertical_line(
                self.center.x + y,
                self.center.y - x,
                self.center.y + x,
                self.bg,
            );
            buffer.part_vertical_line(
                self.center.x - x,
                self.center.y - y,
                self.center.y + y,
                self.bg,
            );
            buffer.part_vertical_line(
                self.center.x - y,
                self.center.y - x,
                self.center.y + x,
                self.bg,
            );

            buffer.put_pixel((self.center.x + x, self.center.y + y).into(), self.fg);
            buffer.put_pixel((self.center.x + y, self.center.y + x).into(), self.fg);
            buffer.put_pixel((self.center.x - x, self.center.y + y).into(), self.fg);
            buffer.put_pixel((self.center.x - y, self.center.y + x).into(), self.fg);
            buffer.put_pixel((self.center.x + x, self.center.y - y).into(), self.fg);
            buffer.put_pixel((self.center.x + y, self.center.y - x).into(), self.fg);
            buffer.put_pixel((self.center.x - x, self.center.y - y).into(), self.fg);
            buffer.put_pixel((self.center.x - y, self.center.y - x).into(), self.fg);
            if d >= x * 2 {
                d -= 2 * x + 1;
                x += 1;
            } else if d < 2 * (self.radius as isize - y) {
                d += 2 * y - 1;
                y -= 1;
            } else {
                d += 2 * (y - x - 1);
                y -= 1;
                x += 1;
            }
        }
    }
}
