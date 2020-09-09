use crate::drawable::Drawable;
use cgmath::Vector2;
use utils::Color;

pub struct Buffer {
    pub size: Vector2<usize>,
    pub(crate) content: Vec<Color>,
}

impl Buffer {
    pub fn new(size: Vector2<usize>, color: Color) -> Self {
        Self {
            size,
            content: vec![color; size.x * size.y],
        }
    }

    pub fn scale(&self, scale: usize) -> Buffer {
        let mut new_content = vec![Color::TRANSPARENT; self.size.x * scale * self.size.y * scale];

        for x in 0..self.size.x {
            for y in 0..self.size.y {
                for x_bis in 0..scale {
                    for y_bis in 0..scale {
                        new_content[(y * scale + y_bis) * self.size.x * scale + x * scale + x_bis] =
                            self.content[y * self.size.x + x]
                    }
                }
            }
        }

        Buffer {
            content: new_content,
            size: (self.size.x * scale, self.size.y * scale).into(),
        }
    }
}

impl Drawable for Buffer {
    fn fill(&mut self, color: Color) {
        self.content.copy_from_slice(
            (0..self.size.x * self.size.y)
                .map(|_| color)
                .collect::<Vec<Color>>()
                .as_ref(),
        );
    }
    fn put_pixel(&mut self, p: Vector2<usize>, color: Color) {
        if p.x < self.size.x && p.y < self.size.y {
            self.content[p.y * self.size.x + p.x] = color;
        }
    }
    fn get_pixel(&self, p: Vector2<usize>) -> Color {
        self.content[p.y * self.size.x + p.x]
    }
    fn horizontal_line(&mut self, y: usize, color: Color) {
        if y < self.size.y {
            self.content[y * self.size.x..((y + 1) * self.size.x)].copy_from_slice(
                (0..self.size.x)
                    .map(|_| color)
                    .collect::<Vec<Color>>()
                    .as_ref(),
            );
        }
    }

    fn part_horizontal_line(&mut self, y: usize, x_start: usize, x_stop: usize, color: Color) {
        if y < self.size.y {
            let x_start = x_start.max(self.size.x);
            let x_stop = x_stop.max(self.size.x);
            self.content[y * self.size.x + x_start..(y * self.size.x + (x_stop))].copy_from_slice(
                (0..(x_stop - x_start))
                    .map(|_| color)
                    .collect::<Vec<Color>>()
                    .as_ref(),
            );
        }
    }

    fn vertical_line(&mut self, x: usize, color: Color) {
        if x < self.size.x {
            for i in 0..self.size.y {
                self.content[i * self.size.x + x] = color;
            }
        }
    }

    fn part_vertical_line(&mut self, x: usize, y_start: usize, y_stop: usize, color: Color) {
        if x < self.size.x {
            let y_start = y_start.min(self.size.y);
            let y_stop = y_stop.min(self.size.y);
            for i in y_start..y_stop {
                self.content[i * self.size.x + x] = color;
            }
        }
    }

    /// TODO: Optimiser la copie ligne par ligne
    fn put_buffer(&mut self, offset: Vector2<usize>, buffer: &Buffer) {
        for x in 0..buffer.size.x {
            for y in 0..buffer.size.y {
                self.put_pixel(
                    (x + offset.x, y + offset.y).into(),
                    buffer.get_pixel((x, y).into()),
                )
            }
        }
    }
}
