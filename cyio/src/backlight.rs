use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;

pub struct Backlight {}

impl Backlight {
    pub fn new() -> Self {
        Self {}
    }
    pub fn set_backlight(&self, v: u8) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .write(true)
            .open("/sys/class/leds/frontlight/brightness")?;
        file.write(format!("{}", v).as_ref())?;
        Ok(())
    }
}
