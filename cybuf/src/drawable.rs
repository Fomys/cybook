use crate::Buffer;
use cgmath::Vector2;
use std::error::Error;
use utils::Color;

pub trait Drawable {
    fn fill(&mut self, color: Color);
    fn put_pixel(&mut self, p: Vector2<usize>, color: Color);
    fn get_pixel(&self, p: Vector2<usize>) -> Color;

    fn horizontal_line(&mut self, y: usize, color: Color);
    fn part_horizontal_line(&mut self, y: usize, x_start: usize, x_stop: usize, color: Color);

    fn vertical_line(&mut self, x: usize, color: Color);
    fn part_vertical_line(&mut self, x: usize, y_start: usize, y_stop: usize, color: Color);

    fn flush(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn draw_line(&mut self, mut start: Vector2<usize>, stop: Vector2<usize>, color: Color) {
        let dx = (stop.x as isize - start.x as isize).abs();
        let sx = start.x < stop.x;
        let dy = -(stop.y as isize - start.y as isize).abs();
        let sy = start.y < stop.y;
        let mut err = dx + dy;
        loop {
            self.put_pixel(start, color);
            if start == stop {
                break;
            };
            let e2 = 2 * err;
            if e2 > dy {
                err += dy;
                if sx {
                    start.x += 1;
                } else {
                    start.x -= 1;
                }
            }
            if e2 < dx {
                err += dx;
                if sy {
                    start.y += 1;
                } else {
                    start.y -= 1;
                }
            }
        }
    }

    fn put_buffer(&mut self, offset: Vector2<usize>, buffer: &Buffer);
}
