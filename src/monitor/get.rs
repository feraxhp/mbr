use ddc_hi::{Ddc, Display};

pub fn get_brightness(display: &mut Display) -> Option<u16> {
    match display.handle.get_vcp_feature(0x10) {
        Ok(val) => Some(val.value()),
        _ => Some(u16::MAX),
    }
}

pub fn get_brightness_id(id: String) -> Option<u16> {
    for mut display in Display::enumerate() {
        if display.info.id.to_lowercase().contains(&id.to_lowercase()) {
            return get_brightness(&mut display);
        }
    }
    
    None
}
