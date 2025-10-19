//! Browser localStorage API bindings.
//!
//! This module provides simple access to the browser's localStorage API
//! for persistent key-value storage.
//!
//! # Example
//!
//! ```no_run
//! use website_base::local_storage::{local_storage_set_item, local_storage_get_item};
//!
//! // Store a value
//! local_storage_set_item("username", "alice");
//!
//! // Retrieve a value
//! if let Some(username) = local_storage_get_item("username") {
//!     println!("Username: {}", username);
//! }
//! ```

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    /// Stores a key-value pair in localStorage.
    ///
    /// # Arguments
    ///
    /// * `key` - The storage key
    /// * `value` - The value to store (must be a string)
    #[wasm_bindgen(js_namespace = window, js_name = "localStorage.setItem")]
    pub fn local_storage_set_item(key: &str, value: &str);

    /// Retrieves a value from localStorage by key.
    ///
    /// # Arguments
    ///
    /// * `key` - The storage key
    ///
    /// # Returns
    ///
    /// Returns `Some(String)` if the key exists, `None` otherwise.
    #[wasm_bindgen(js_namespace = window, js_name = "localStorage.getItem")]
    pub fn local_storage_get_item(key: &str) -> Option<String>;
}