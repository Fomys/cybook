use crate::drawable::Drawable;
use crate::Buffer;
use cgmath::Vector2;
use memmap::{MmapMut, MmapOptions};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::path::Path;
use utils::{Color, SCREEN_SIZE};

pub struct Framebuffer<'a> {
    #[allow(dead_code)]
    path: &'a Path,
    #[allow(dead_code)]
    device: File,
    pub frame: MmapMut,
}

impl<'a> Framebuffer<'a> {
    pub fn new(path: &'a Path) -> Result<Self, Box<dyn Error>> {
        let device = OpenOptions::new().write(true).read(true).open(path)?;
        let frame_length = utils::SCREEN_SIZE.x * utils::SCREEN_SIZE.y;
        let frame = unsafe {
            MmapOptions::new()
                .len(frame_length as usize)
                .map_mut(&device)
        }?;
        Ok(Self {
            device,
            frame,
            path,
        })
    }

    pub fn write_frame(&mut self, frame: &[u8]) -> Result<(), Box<dyn Error>> {
        self.frame[..].copy_from_slice(&frame);
        Ok(())
    }

    pub fn update(&mut self) -> Result<(), Box<dyn Error>> {
        match self.frame.flush() {
            Err(e) => {
                if e.raw_os_error() != Some(0) {
                    Err(Box::new(e))
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }
}

impl<'a> Drawable for Framebuffer<'a> {
    fn fill(&mut self, color: Color) {
        self.frame[..].copy_from_slice(
            (0..SCREEN_SIZE.x * SCREEN_SIZE.y)
                .map(|_| color.g)
                .collect::<Vec<u8>>()
                .as_ref(),
        );
    }

    fn put_pixel(&mut self, p: Vector2<isize>, color: Color) {
        if p.x < SCREEN_SIZE.x && p.y < SCREEN_SIZE.y {
            self.frame[(p.y * SCREEN_SIZE.x + p.x) as usize] = color.g;
        }
    }

    fn get_pixel(&self, p: Vector2<isize>) -> Color {
        Color {
            g: self.frame[(p.y * SCREEN_SIZE.x + p.x) as usize],
            a: 255,
        }
    }

    fn horizontal_line(&mut self, y: isize, color: Color) {
        if y < SCREEN_SIZE.y {
            self.frame[(y * SCREEN_SIZE.x) as usize..(((y + 1) * SCREEN_SIZE.x) as usize)]
                .copy_from_slice(
                    (0..SCREEN_SIZE.x)
                        .map(|_| color.g)
                        .collect::<Vec<u8>>()
                        .as_ref(),
                );
        }
    }

    fn part_horizontal_line(&mut self, y: isize, x_start: isize, x_stop: isize, color: Color) {
        if 0 < y && y < SCREEN_SIZE.y {
            let x_start = x_start.max(SCREEN_SIZE.x);
            let x_stop = x_stop.max(SCREEN_SIZE.x);
            self.frame
                [(y * SCREEN_SIZE.x + x_start) as usize..((y * SCREEN_SIZE.x + (x_stop)) as usize)]
                .copy_from_slice(
                    (0..(x_stop - x_start))
                        .map(|_| color.g)
                        .collect::<Vec<u8>>()
                        .as_ref(),
                );
        }
    }

    fn vertical_line(&mut self, x: isize, color: Color) {
        if x < SCREEN_SIZE.x {
            for i in 0..SCREEN_SIZE.y {
                self.frame[(i * SCREEN_SIZE.x + x) as usize] = color.g;
            }
        }
    }

    fn part_vertical_line(&mut self, x: isize, y_start: isize, y_stop: isize, color: Color) {
        if x < SCREEN_SIZE.x {
            let y_start = y_start.min(SCREEN_SIZE.y);
            let y_stop = y_stop.min(SCREEN_SIZE.y);
            for i in y_start..y_stop {
                self.frame[(i * SCREEN_SIZE.x + x) as usize] = color.g;
            }
        }
    }

    fn flush(&mut self) -> Result<(), Box<dyn Error>> {
        self.frame.flush()?;
        Ok(())
    }

    /// TODO: Optimiser la copie ligne par ligne
    fn put_buffer(&mut self, offset: Vector2<isize>, buffer: &Buffer) {
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
