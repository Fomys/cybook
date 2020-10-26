use crate::{
    widget::{Frame, Widget},
    Event, Handler,
};
use cybuf::{Drawable, Framebuffer};
use cyio::Input;
use std::{
    error::Error,
    path::Path,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use utils::Color;

pub struct Window<'f, T> {
    framebuffer: Framebuffer<'f>,
    rx: mpsc::Receiver<Event<T>>,
    pub tx: mpsc::Sender<Event<T>>,
    pub frame: Frame<Framebuffer<'f>, T>,
}

impl<'f, T> Window<'f, T>
where
    T: Clone + Send + 'static,
{
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let (tx, rx) = mpsc::channel();
        let mut framebuffer = Framebuffer::new(Path::new("/dev/fb0"))?;
        framebuffer.fill(Color::WHITE);
        framebuffer.update()?;
        let mut frame = Frame::new();
        frame.attach(tx.clone());
        Ok(Self {
            framebuffer,
            rx,
            frame,
            tx,
        })
    }

    pub fn mainloop(&mut self) -> Result<(), Box<dyn Error>> {
        let mut input = Input::new()?;
        let i_tx = self.tx.clone();
        thread::spawn(move || loop {
            match input.get_event() {
                Ok(event) => {
                    i_tx.send(Event::IO(event)).unwrap_or(());
                }
                _ => {}
            }
        });
        let mut last_update = Instant::now();
        loop {
            for event in self.rx.try_iter() {
                self.frame.handle_event(event);
            }
            if last_update.elapsed() > Duration::from_millis(500) {
                self.frame.draw(&mut self.framebuffer);
                self.framebuffer.update()?;
                last_update = Instant::now();
            }
        }
        Ok(())
    }
}
