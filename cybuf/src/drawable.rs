use crate::Buffer;
use cgmath::Vector2;
use std::error::Error;
use utils::Color;

pub trait Drawable {
    fn fill(&mut self, color: Color);
    fn put_pixel(&mut self, p: Vector2<isize>, color: Color);
    fn get_pixel(&self, p: Vector2<isize>) -> Color;

    fn horizontal_line(&mut self, y: isize, color: Color);
    fn part_horizontal_line(&mut self, y: isize, x_start: isize, x_stop: isize, color: Color);

    fn vertical_line(&mut self, x: isize, color: Color);
    fn part_vertical_line(&mut self, x: isize, y_start: isize, y_stop: isize, color: Color);

    fn flush(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn draw_line(&mut self, mut start: Vector2<isize>, stop: Vector2<isize>, color: Color) {
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

    fn put_buffer(&mut self, offset: Vector2<isize>, buffer: &Buffer);
}
