use cybuf::Drawable;

pub trait Widget<T: Drawable> {
    fn draw(&mut self, buffer: &mut T);
}
