use crate::base_result::BaseResult;

pub fn get_window_location() -> BaseResult<web_sys::Location> {
    let window = web_sys::window();

    if let Some(window) = window {
        Ok(window.location())
    } else {
        Err("window not found".into())
    }
}