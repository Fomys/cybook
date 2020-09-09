use crate::Event;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read};

/// Input handler
pub struct Input {
    reader: BufReader<File>,
}

impl Input {
    /// Create a new input handler
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let file = OpenOptions::new().read(true).open("/dev/cyio")?;
        let reader = BufReader::new(file);
        Ok(Self { reader })
    }

    /// Get next event
    pub fn get_event(&mut self) -> Result<Event, Box<dyn Error>> {
        let mut buf = [0u8; 16];
        self.reader.read_exact(&mut buf)?;
        Ok(buf.into())
    }
}
