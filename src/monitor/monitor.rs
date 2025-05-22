use std::{io, u16};
use std::io::Write;

pub struct Monitor {
    name: String,
    brightness: Option<u16>,
}

impl Monitor {
    pub fn new(name: String) -> Monitor {
        Monitor {
            name: name,
            brightness: None
        }
    }
    
    pub fn set_brightness(&mut self, brightness: Option<u16>) {
        self.brightness = brightness;
    }
    
    pub fn show(monitors: Vec<Monitor>, head: bool) {
        let show_brightness = monitors.iter().any(|m| m.brightness.is_some());
        
        // Calculate column widths
        let max_name_len = monitors.iter().map(|m| m.name.len()).max().unwrap_or(4);

        let max_name_len = std::cmp::max(4, max_name_len);
        let max_brightness_len = if show_brightness { 10 } else { 0 };
        
        if head {
            let _ = io::stdout().write(
                format!(
                    "{0: <width$} | {1: <brightness$}\n",
                    "Name",
                    if show_brightness { "Brightness" } else { "" },
                    width = max_name_len,
                    brightness = max_brightness_len
                )
                .as_bytes(),
            );
        }

        // Print each monitor
        for monitor in monitors {
            let divider = match head {
                false => "",
                true => "|"
            };
            
            let brightness = match monitor.brightness {
                Some(br) => match br {
                    u16::MAX => continue,
                    br => br.to_string()
                },
                None => "".to_string()
            };
            
            let _ = io::stdout().write(
                format!(
                    "{0: <width$} {1} {2: <brightness$}\n",
                    monitor.name,
                    divider,
                    brightness,
                    width = max_name_len,
                    brightness = max_brightness_len
                )
                .as_bytes(),
            );
        }

        let _ = io::stdout().flush();
    }
}
