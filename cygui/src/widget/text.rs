use crate::event::EventState;
use crate::widget::Widget;
use crate::{Event, Handler};
use cgmath::Vector2;
use cybuf::{Buffer, Drawable};
use fontdue::Font;
use utils::Color;

pub struct Text<'f> {
    text: String,
    buffer: Buffer,
    position: Vector2<usize>,
    scale: f32,
    font: &'f Font,
    color: Color,
}

impl<'f> Text<'f> {
    pub fn new(
        text: String,
        position: Vector2<usize>,
        scale: f32,
        font: &'f Font,
        color: Color,
    ) -> Self {
        let buffer = Buffer::new((1, 1).into(), Color::TRANSPARENT);
        let mut text = Self {
            position,
            text,
            scale,
            buffer,
            font,
            color,
        };
        text.pre_render();
        text
    }

    pub fn pre_render(&mut self) {
        let mut letters: Vec<Buffer> = vec![];
        let mut size = Vector2 {
            x: 0usize,
            y: 0usize,
        };
        for letter in self.text.chars() {
            let (metrics, letter) = self.font.rasterize(letter, self.scale, 0.0);
            size.x += metrics.width;
            size.y = size.y.max(metrics.height);
            let mut temp_buffer =
                Buffer::new((metrics.width, metrics.height).into(), Color::TRANSPARENT);
            for x in 0..metrics.height {
                for y in 0..metrics.width {
                    if letter[y * metrics.width + x] != 0 {
                        temp_buffer.put_pixel((x, y).into(), self.color);
                    }
                }
            }
            letters.push(temp_buffer);
        }
        let mut buffer = Buffer::new(size, Color::TRANSPARENT);
        let mut offset = 0usize;
        for letter in letters {
            buffer.put_buffer((offset, 0).into(), &letter);
            offset += letter.size.x;
        }
        self.buffer = buffer;
    }
}

impl<'f, T> Widget<T> for Text<'f>
where
    T: Drawable,
{
    fn draw(&mut self, buffer: &mut T) {
        buffer.put_buffer(self.position, &self.buffer);
    }
}

impl<'f> Handler for Text<'f> {
    fn handle_event(&mut self, _: &dyn Event) -> EventState {
        EventState::Pending
    }
}
