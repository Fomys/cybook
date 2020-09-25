use crate::Handler;
use cybuf::Drawable;

pub trait Widget<D: Drawable, T: Clone>: Handler<T> {
    fn draw(&self, buffer: &mut D);
}
