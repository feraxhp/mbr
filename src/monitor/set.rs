use ddc_hi::{Ddc, Display};

pub fn set_brigthness(id: Option<String>, level: u16) -> Result<u16, Result<Vec<String>, String>> {
    let mut erros = Vec::new();
    
    for mut display in Display::enumerate() {
        if id.as_ref().map_or(true, |id| display.info.id.to_lowercase().contains(&id.to_lowercase())) {
            match display.handle.set_vcp_feature(0x10, level) {
                Ok(_) => return Ok(level),
                Err(e) => return Err(Err(format!("{:?}", e))),
            }
        } 
        
        if id.is_none() {
            match display.handle.set_vcp_feature(0x10, level) {
                Ok(_) => {},
                Err(e) => erros.push(format!("{:?}", e)),
            }
        }
    }
    
    if !erros.is_empty() { Err(Ok(erros)) } 
    else { Ok(level) }
}