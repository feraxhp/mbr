use ddc_hi::{Display};

use super::{get::get_brightness, monitor::Monitor};

pub enum Config {
    BRIGHTNESS,
}

pub fn list_monitors(options: Vec<Config>) -> Vec<Monitor> {
    let mut monitors: Vec<Monitor> = Vec::new();
    
    let brightness = options.iter().any(|option| matches!(option, Config::BRIGHTNESS));

    
    for mut display in Display::enumerate() {
        let mut monitor = Monitor::new(display.info.id.to_string());
        if brightness {
            let br = get_brightness(&mut display);
            monitor.set_brightness(br)
        }
        
        monitors.push(monitor);
    }
    
    monitors
}