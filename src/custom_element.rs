//! Custom element (Web Components) registration utilities.
//!
//! This module provides functionality for defining and registering custom HTML elements
//! using the Web Components API.
//!
//! # Example
//!
//! ```no_run
//! use website_base::custom_element::{define_custom_element, CustomElementDefinition};
//!
//! // Define a custom element with a connected callback
//! let definition = CustomElementDefinition {
//!     element_name: "my-element".to_string(),
//!     connected_callback_function_name: "onMyElementConnected".to_string(),
//! };
//!
//! define_custom_element(&definition).unwrap();
//! ```

use crate::base_result::{BaseResult, ToBaseResult};
use js_sys::eval;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

/// Configuration for defining a custom element.
///
/// Contains the element name and the name of the JavaScript function
/// to call when the element is connected to the DOM.
pub struct CustomElementDefinition {
    /// The tag name for the custom element (must contain a hyphen, e.g., "my-element")
    pub element_name: String,
    /// Name of the JavaScript function to call in connectedCallback
    pub connected_callback_function_name: String,
}

/// Defines and registers a custom element with the browser.
///
/// Creates a custom element class that extends HTMLElement and registers it
/// with the CustomElementRegistry. The element will call the specified function
/// when connected to the DOM.
///
/// # Arguments
///
/// * `custom_element_definition` - Configuration for the custom element
///
/// # Errors
///
/// Returns an error if:
/// - The element name is invalid (doesn't contain a hyphen)
/// - The element name is already registered
/// - JavaScript evaluation fails
///
/// # Example
///
/// ```no_run
/// use website_base::custom_element::{define_custom_element, CustomElementDefinition};
///
/// let definition = CustomElementDefinition {
///     element_name: "hello-world".to_string(),
///     connected_callback_function_name: "initHelloWorld".to_string(),
/// };
///
/// define_custom_element(&definition)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn define_custom_element(custom_element_definition: &CustomElementDefinition) -> BaseResult<()> {
    let name = &custom_element_definition.element_name;

    let class = format!("(class BstructCustomElement extends HTMLElement {{ constructor(){{super();}} connectedCallback(){{ {}(this); }} }})", custom_element_definition.connected_callback_function_name);
    let class = eval(&class).to_base_result()?;

    custom_elements_define(name, &class).to_base_result()
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = customElements, js_name = "define")]
    fn custom_elements_define(name: &str, constructor: &JsValue) -> Result<(), JsValue>;
}
