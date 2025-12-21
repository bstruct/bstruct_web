//! Window and location utilities.
//!
//! This module provides utilities for accessing the browser window object
//! and its location property.
use crate::base_result::BaseResult;

/// Gets the window's location object.
///
/// The Location object contains information about the current URL
/// and provides methods for navigation.
///
/// # Errors
///
/// Returns an error if the window is not available.
///
/// # Example
///
/// ```no_run
/// use website_base::window::get_window_location;
///
/// let location = get_window_location().unwrap();
/// let pathname = location.pathname().unwrap();
/// ```
pub fn get_window_location() -> BaseResult<web_sys::Location> {
    let window = web_sys::window();

    if let Some(window) = window {
        Ok(window.location())
    } else {
        Err("window not found".into())
    }
}