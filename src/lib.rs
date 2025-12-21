//! # website_base
//!
//! A Rust/WebAssembly library providing high-level abstractions for web browser APIs.
//! This library simplifies common web development tasks such as DOM manipulation,
//! custom elements, API calls, and local storage operations.
//!
//! ## Features
//!
//! - **Document manipulation**: Create and manage HTML elements with a type-safe API
//! - **Custom Elements**: Define and register Web Components
//! - **API calls**: Make HTTP requests with automatic JSON handling
//! - **Local Storage**: Simple key-value storage interface
//! - **Navigation**: Handle browser navigation events
//! - **Window utilities**: Access window and location objects

mod error_messages;
pub mod api_call;
pub mod base_result;
pub mod custom_element;
pub mod document;
pub mod local_storage;
pub mod navigation;
pub mod struct_node;
pub mod window;
pub use web_sys;
pub use serde_wasm_bindgen;
pub use wasm_bindgen;
