use crate::event::IOEvent;
use crate::widget::{Frame, Widget};
use crate::{Event, Handler};
use cybuf::Framebuffer;
use cyio::Input;
use std::error::Error;
use std::path::Path;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::{Duration, Instant};

pub struct Window<'f> {
    pub tx: Sender<Box<dyn Event>>,
    rx: Receiver<Box<dyn Event>>,
    framebuffer: Framebuffer<'f>,
    frame: Frame<Framebuffer<'f>>,
}

impl<'f> Window<'f> {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let (tx, rx) = mpsc::channel::<Box<dyn Event>>();
        let framebuffer = Framebuffer::new(Path::new("/dev/fb0"))?;
        let frame = Frame::new();
        Ok(Self {
            tx,
            rx,
            framebuffer,
            frame,
        })
    }

    pub fn mainloop(&mut self) -> Result<(), Box<dyn Error>> {
        let mut last_update = Instant::now();
        loop {
            self.frame.handle_event(self.rx.recv()?.as_ref());
            if last_update.elapsed() > Duration::from_millis(300) {
                self.frame.draw(&mut self.framebuffer);
                last_update = Instant::now();
            }
        }
        Ok(())
    }

    fn event_loop(tx: Sender<Box<dyn Event>>) -> Result<(), Box<dyn Error>> {
        let mut input = Input::new()?;
        loop {
            let ioevent: Box<IOEvent> = Box::new(input.get_event()?.into());
            tx.send(ioevent);
        }
        Ok(())
    }
}
