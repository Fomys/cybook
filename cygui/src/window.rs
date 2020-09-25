use crate::widget::{Frame, Widget};
use cybuf::{Drawable, Framebuffer};

use std::error::Error;
use std::path::Path;

use crate::Handler;
use std::sync::mpsc;
use std::thread::sleep;
use std::time::{Duration, Instant};
use utils::Color;

pub struct Window<'f, T> {
    framebuffer: Framebuffer<'f>,
    rx: mpsc::Receiver<T>,
    pub tx: mpsc::Sender<T>,
    pub frame: Frame<Framebuffer<'f>, T>,
}

impl<'f, T> Window<'f, T>
where
    T: Clone + Send + 'static,
{
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let (tx, rx) = mpsc::channel();
        let framebuffer = Framebuffer::new(Path::new("/dev/fb0"))?;
        let frame = Frame::new();
        Ok(Self {
            framebuffer,
            rx,
            frame,
            tx,
        })
    }

    pub fn mainloop(&mut self) -> Result<(), Box<dyn Error>> {
        let mut last_update = Instant::now();
        loop {
            for event in self.rx.try_iter() {
                self.frame.handle_event(event);
            }
            if last_update.elapsed() > Duration::from_millis(300) {
                self.frame.draw(&mut self.framebuffer);
                self.framebuffer.update()?;
                last_update = Instant::now();
            }
        }
        Ok(())
    }
}
