use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_test::*;
use website_base::{
    custom_element::{define_custom_element, get_custom_element, CustomElementDefinition},
    document::{create_element, HtmlNode},
};

#[allow(dead_code)]
#[wasm_bindgen_test]
fn test_define_custom_element_invalid_name() {
    // Create a custom element constructor
    let definition = CustomElementDefinition {
        element_name: "invalidelement".to_string(),
        connected_callback_function_name: "connected_callback".to_string(),
    };

    // Test defining a custom element with an invalid name (should not contain hyphen according to spec)
    // But let's test with a name that would cause issues
    let result = define_custom_element(&definition);

    // This should fail because custom element names must contain a hyphen
    assert!(
        result.is_err(),
        "Expected Err result for invalid element name"
    );
}

#[allow(dead_code)]
#[wasm_bindgen_test]
fn test_define_custom_element_duplicate_name() {
    // Create a custom element constructor
    let definition = CustomElementDefinition {
        element_name: "test-duplicate-1".to_string(),
        connected_callback_function_name: "connected_callback".to_string(),
    };

    // Define the first element
    let result1 = define_custom_element(&definition);
    assert!(result1.is_ok(), "Expected first definition to succeed");

    // Try to define another element with the same name
    let result2 = define_custom_element(&definition);

    // This should fail because the name is already taken
    assert!(
        result2.is_err(),
        "Expected Err result for duplicate element name"
    );
}

#[allow(dead_code)]
#[wasm_bindgen_test]
fn test_define_custom_element_empty_name() {
    // Create a custom element constructor
    let definition = CustomElementDefinition {
        element_name: "".to_string(),
        connected_callback_function_name: "connected_callback".to_string(),
    };

    // Test defining a custom element with an empty name
    let result = define_custom_element(&definition);

    // This should fail
    assert!(
        result.is_err(),
        "Expected Err result for empty element name"
    );
}

#[allow(dead_code)]
#[wasm_bindgen_test]
fn test_define_custom_rust_type() {
    #[wasm_bindgen]
    pub fn connected_callback(element: &web_sys::Element) {
        element.set_inner_html(&format!(
            "Hello from custom element, {}!",
            &element.tag_name()
        ));
    }

    let definition = CustomElementDefinition {
        element_name: "bstruct-table".to_string(),
        connected_callback_function_name: "connected_callback".to_string(),
    };

    // Test defining a custom element
    let result = define_custom_element(&definition);
    assert!(result.is_ok());

    //add element of that type to the document to trigger connectedCallback
    let document = HtmlNode::get_document()
        .unwrap()
        .to_document_node()
        .unwrap();
    let body = document.body().unwrap();
    let element = create_element(&definition.element_name).unwrap();
    body.append_child(&element.to_node().unwrap()).unwrap();
}

#[allow(dead_code)]
#[wasm_bindgen_test]
fn test_get_custom_element_not_registered() {
    // Test getting a custom element that hasn't been registered
    let result = get_custom_element("nonexistent-element");
    
    assert!(result.is_ok(), "Expected Ok result");
    assert!(result.unwrap().is_none(), "Expected None for unregistered element");
}

#[allow(dead_code)]
#[wasm_bindgen_test]
fn test_get_custom_element_after_registration() {
    #[wasm_bindgen]
    pub fn get_test_callback(element: &web_sys::Element) {
        element.set_inner_html("Test element for get_custom_element");
    }

    let definition = CustomElementDefinition {
        element_name: "get-test-element".to_string(),
        connected_callback_function_name: "get_test_callback".to_string(),
    };

    // First, check that it doesn't exist
    let before_result = get_custom_element(&definition.element_name);
    assert!(before_result.is_ok(), "Expected Ok result before registration");
    assert!(before_result.unwrap().is_none(), "Expected None before registration");

    // Register the custom element
    let define_result = define_custom_element(&definition);
    assert!(define_result.is_ok(), "Expected successful registration");

    // Now check that it exists
    let after_result = get_custom_element(&definition.element_name);
    assert!(after_result.is_ok(), "Expected Ok result after registration");
    assert!(after_result.unwrap().is_some(), "Expected Some after registration");
}

#[allow(dead_code)]
#[wasm_bindgen_test]
fn test_get_custom_element_multiple_elements() {
    #[wasm_bindgen]
    pub fn multi_test_callback_1(element: &web_sys::Element) {
        element.set_inner_html("Element 1");
    }

    #[wasm_bindgen]
    pub fn multi_test_callback_2(element: &web_sys::Element) {
        element.set_inner_html("Element 2");
    }

    let definition1 = CustomElementDefinition {
        element_name: "multi-test-element-1".to_string(),
        connected_callback_function_name: "multi_test_callback_1".to_string(),
    };

    let definition2 = CustomElementDefinition {
        element_name: "multi-test-element-2".to_string(),
        connected_callback_function_name: "multi_test_callback_2".to_string(),
    };

    // Register first element
    define_custom_element(&definition1).unwrap();
    
    // Check first element exists
    let result1 = get_custom_element(&definition1.element_name).unwrap();
    assert!(result1.is_some(), "Expected first element to be registered");

    // Check second element doesn't exist yet
    let result2_before = get_custom_element(&definition2.element_name).unwrap();
    assert!(result2_before.is_none(), "Expected second element to not be registered yet");

    // Register second element
    define_custom_element(&definition2).unwrap();

    // Check both elements exist
    let result1_after = get_custom_element(&definition1.element_name).unwrap();
    let result2_after = get_custom_element(&definition2.element_name).unwrap();
    assert!(result1_after.is_some(), "Expected first element to still be registered");
    assert!(result2_after.is_some(), "Expected second element to be registered");
}

#[allow(dead_code)]
#[wasm_bindgen_test]
fn test_get_custom_element_conditional_registration() {
    #[wasm_bindgen]
    pub fn conditional_test_callback(element: &web_sys::Element) {
        element.set_inner_html("Conditional element");
    }

    let definition = CustomElementDefinition {
        element_name: "conditional-test-element".to_string(),
        connected_callback_function_name: "conditional_test_callback".to_string(),
    };

    // Check if element exists before registering
    if get_custom_element(&definition.element_name).unwrap().is_none() {
        // Register only if it doesn't exist
        let result = define_custom_element(&definition);
        assert!(result.is_ok(), "Expected successful registration");
    }

    // Verify it now exists
    let final_result = get_custom_element(&definition.element_name).unwrap();
    assert!(final_result.is_some(), "Expected element to be registered");

    // Try to register again using the same pattern - should be idempotent
    if get_custom_element(&definition.element_name).unwrap().is_none() {
        define_custom_element(&definition).unwrap();
    }

    // Should still exist
    let final_result2 = get_custom_element(&definition.element_name).unwrap();
    assert!(final_result2.is_some(), "Expected element to still be registered");
}
#[allow(dead_code)]
#[wasm_bindgen_test]
fn test_define_custom_rust_type_2() {
    #[wasm_bindgen]
    pub fn connected_callback_2(element: &web_sys::Element) {
        element.set_inner_html(&format!(
            "Hello again from custom element, {}!",
            &element.tag_name()
        ));
    }

    let definition = CustomElementDefinition {
        element_name: "bstruct-table-2".to_string(),
        connected_callback_function_name: "connected_callback_2".to_string(),
    };

    // Test defining a custom element
    let result = define_custom_element(&definition);
    assert!(result.is_ok());

    //add element of that type to the document to trigger connectedCallback
    let document = HtmlNode::get_document()
        .unwrap()
        .to_document_node()
        .unwrap();
    let body = document.body().unwrap();
    let element = create_element(&definition.element_name).unwrap();
    body.append_child(&element.to_node().unwrap()).unwrap();
}
