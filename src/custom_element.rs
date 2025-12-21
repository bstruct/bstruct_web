//! Custom element (Web Components) registration utilities.
//!
//! This module provides functionality for defining and registering custom HTML elements
//! using the Web Components API.
//!
//! # Example
//!
//! ```no_run
//! use website_base::custom_element::CustomElementDefinition;
//!
//! // Define a custom element and automatically register it if not already registered
//! let definition = CustomElementDefinition {
//!     tag_name: "my-element".to_string(),
//! };
//!
//! definition.check_and_define().unwrap();
//! ```

use crate::base_result::{BaseResult, ToBaseResult};
use js_sys::eval;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

/// Configuration for defining a custom element.
///
/// A lightweight wrapper for custom element registration that only requires
/// a tag name. The custom element will be a basic HTMLElement subclass with
/// no custom behavior.
pub struct CustomElementDefinition {
    /// The tag name for the custom element (must contain a hyphen, e.g., "my-element")
    pub tag_name: String,
}

impl CustomElementDefinition {
    /// Checks if the custom element is already registered and defines it if not.
    ///
    /// This is a convenience method that combines checking for existing registration
    /// and defining the element in a single call, consuming and returning self for
    /// method chaining.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - JavaScript evaluation fails
    /// - The custom element cannot be registered
    ///
    /// # Example
    ///
    /// ```no_run
    /// use website_base::custom_element::CustomElementDefinition;
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let definition = CustomElementDefinition {
    ///     tag_name: "hello-world".to_string(),
    /// }.check_and_define()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn check_and_define(self) -> BaseResult<CustomElementDefinition> {
        if get_custom_element(&self.tag_name)?.is_none() {
            define_custom_element(&self)?;
        }

        Ok(self)
    }
}

/// Defines and registers a custom element with the browser.
///
/// Creates a basic custom element class that extends HTMLElement and registers it
/// with the CustomElementRegistry. The created element has no custom behavior and
/// is primarily useful as a semantic container or styling hook.
///
/// # Arguments
///
/// * `custom_element_definition` - Configuration containing the tag name
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
/// use website_base::custom_element::{define_custom_element, get_custom_element, CustomElementDefinition};
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let definition = CustomElementDefinition {
///     tag_name: "hello-world".to_string(),
/// };
///
/// // Check if already registered to avoid errors
/// if get_custom_element(&definition.tag_name)?.is_none() {
///     define_custom_element(&definition)?;
/// }
/// # Ok(())
/// # }
/// ```
pub fn define_custom_element(
    custom_element_definition: &CustomElementDefinition,
) -> BaseResult<()> {
    // let class = format!("(class BstructCustomElement extends HTMLElement {{ constructor(){{super();}} connectedCallback(){{ {}(this); }} }})", custom_element_definition.connected_callback_function_name);
    let class = "(class BstructCustomElement extends HTMLElement{constructor(){super();}})";
    let class = eval(&class).to_base_result()?;

    custom_elements_define(&custom_element_definition.tag_name, &class).to_base_result()
}

/// Retrieves a custom element constructor from the CustomElementRegistry.
///
/// Returns the constructor for a previously defined custom element, or `None`
/// if no element with the given name has been registered.
///
/// # Arguments
///
/// * `element_name` - The tag name of the custom element to retrieve
///
/// # Errors
///
/// Returns an error if there's a JavaScript error accessing the registry.
///
/// # Example
///
/// ```no_run
/// use website_base::custom_element::get_custom_element;
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Check if a custom element is registered
/// if let Some(constructor) = get_custom_element("my-element")? {
///     // Element is registered
///     println!("Custom element 'my-element' is registered");
/// } else {
///     println!("Custom element 'my-element' is not registered");
/// }
/// # Ok(())
/// # }
/// ```
pub fn get_custom_element(element_name: &str) -> BaseResult<Option<JsValue>> {
    let result = custom_elements_get(element_name).to_base_result()?;

    if result.is_undefined() {
        Ok(None)
    } else {
        Ok(Some(result))
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = customElements, js_name = "define")]
    fn custom_elements_define(name: &str, constructor: &JsValue) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, js_namespace = customElements, js_name = "get")]
    fn custom_elements_get(name: &str) -> Result<JsValue, JsValue>;
}
