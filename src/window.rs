use std::error;

pub fn get_window_location() -> Result<web_sys::Location, Box<dyn error::Error>> {
    let window = web_sys::window();

    if let Some(window) = window {
        Ok(window.location())
    } else {
        Err("window not found".into())
    }
}