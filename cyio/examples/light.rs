use cyio::Backlight;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

pub fn main() -> Result<(), Box<dyn Error>> {
    let backlight = Backlight::new();

    loop {
        for i in 0..255 {
            backlight.set_backlight(i)?;
            sleep(Duration::from_millis(10));
        }
        for i in (0..255).rev() {
            backlight.set_backlight(i)?;
            sleep(Duration::from_millis(10));
        }
    }
}
