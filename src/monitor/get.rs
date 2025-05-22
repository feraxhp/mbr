use ddc_hi::{Ddc, Display};

pub fn get_brightness(display: &mut Display) -> Option<u16> {
    match display.handle.get_vcp_feature(0x10) {
        Ok(val) => Some(val.value()),
        _ => Some(u16::MAX),
    }
}