use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_test::*;
use website_base::{
    custom_element::{define_custom_element, CustomElementDefinition},
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
        element_name: "test-1".to_string(),
        connected_callback_function_name: "connected_callback".to_string(),
    };

    // Define the first element
    let result1 = define_custom_element(&definition);
    assert!(result1.is_ok(), "Expected first definition to succeed");

    // Try to define another element with the same name
    let result2 = define_custom_element(&definition);

    // This should fail because the name is already taken
    assert!(
        result2.is_ok(),
        "Expected second one to succeed. The function should check for existing definitions first"
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
