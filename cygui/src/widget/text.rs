use crate::widget::Widget;

use crate::Handler;
use cgmath::Vector2;
use cybuf::{Buffer, Drawable};
use fontdue::Font;
use std::marker::PhantomData;
use std::rc::Rc;
use utils::Color;

pub struct Text<D, T>
where
    D: Drawable,
    T: Clone,
{
    text: String,
    buffer: Buffer,
    position: Vector2<usize>,
    size: f32,
    font: Rc<Font>,
    fg: Color,
    bg: Color,
    _ph: PhantomData<D>,
    _ph_: PhantomData<T>,
}

impl<D, T> Text<D, T>
where
    D: Drawable,
    T: Clone,
{
    pub fn new(
        text: String,
        position: Vector2<usize>,
        size: f32,
        font: Rc<Font>,
        fg: Color,
        bg: Color,
    ) -> Self {
        let buffer = Buffer::new((1, 1).into(), Color::TRANSPARENT);
        let mut text = Self {
            position,
            text,
            size,
            buffer,
            font,
            fg,
            bg,
            _ph: Default::default(),
            _ph_: Default::default(),
        };
        text.pre_render();
        text
    }

    pub fn update_text(&mut self, text: String) {
        self.text = text;
        self.pre_render();
    }

    pub fn pre_render(&mut self) {
        let mut letters = vec![];
        let mut size = Vector2 {
            x: 0usize,
            y: 0usize,
        };
        for letter in self.text.chars() {
            let (metrics, letter) = self.font.rasterize(letter, self.size);
            size.x += metrics.width;
            size.y = size.y.max(metrics.height);
            let offset = size.y - metrics.height;
            let mut temp_buffer = Buffer::new((metrics.width, metrics.height).into(), self.bg);
            for x in 0..metrics.width {
                for y in 0..metrics.height {
                    if letter[y * metrics.width + x] != 0 {
                        temp_buffer.put_pixel((x, y).into(), self.fg);
                    }
                }
            }
            letters.push((offset, temp_buffer));
        }
        let mut buffer = Buffer::new(size, self.bg);
        let mut x_offset = 0usize;
        for (y_offset, letter) in letters {
            buffer.put_buffer((x_offset, y_offset).into(), &letter);
            x_offset += letter.size.x;
        }
        self.buffer = buffer;
    }
}

impl<D, T> Widget<D, T> for Text<D, T>
where
    D: Drawable,
    T: Clone,
{
    fn draw(&self, buffer: &mut D) {
        buffer.put_buffer(self.position, &self.buffer);
    }
}

impl<D, T: Clone> Handler<T> for Text<D, T>
where
    D: Drawable,
    T: Clone,
{
    fn handle_event(&mut self, _: T) {}
}
