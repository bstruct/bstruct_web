use crate::error_messages::ErrorMessages;
use web_sys::Node;

use wasm_bindgen::prelude::*;

// Change the alias to use `Box<dyn error::Error>`.
use std::error;
// type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[wasm_bindgen]
extern "C" {
    // #[wasm_bindgen(js_namespace = document)]
    // fn getElementsByTagName(qualifiedName: &str) -> Vec<Element>;

    #[wasm_bindgen(catch, js_namespace = document, js_name = "createElement")]
    fn internal_create_element(tag_name: &str) -> Result<web_sys::Element, JsValue>;
    
    // #[wasm_bindgen(js_namespace = document)]
    // fn getElementById(elementId: &str) -> Option<Element>;

    // //https://developer.mozilla.org/en-US/docs/Web/API/Intersection_Observer_API
    // #[wasm_bindgen(js_namespace = document, js_name = "observeElement")]
    // fn observe_element(element: &Element);
    // #[wasm_bindgen(js_namespace = document, js_name = "setState", catch)]
    // fn set_state(state_json: &str) -> Result<(), JsValue>;
}

pub fn create_element(tag_name: &str) -> Result<web_sys::Element, Box<dyn error::Error>> {
    match internal_create_element(tag_name) {
        Ok(e) => Ok(e),
        Err(error) => Err(format!("{:?}", error).into()),
    }
}

pub struct InitialSetup {
    pub title: String,
    pub head_nodes: Vec<Node>,
    pub body_nodes: Vec<Node>,
}

pub fn initial_setup(setup: &InitialSetup) -> Result<web_sys::Document, Box<dyn error::Error>> {
    let window = web_sys::window().expect(&ErrorMessages::not_found("window"));
    let document = window
        .document()
        .expect(&ErrorMessages::not_found("document"));

    let head = document.head().expect(&ErrorMessages::not_found("head"));

    let query_title = head.query_selector("title");
    if let Ok(query_title) = query_title {
        if let Some(title) = query_title {
            title.set_text_content(Some(&setup.title));
        } else {
            let title = document
                .create_element("title")
                .expect(&ErrorMessages::failed_to_create("title"));
            title.set_text_content(Some(&setup.title));

            head.append_child(&title).unwrap();
        }
    }

    let body = document.body().expect(&ErrorMessages::not_found("body"));

    for body_node in &setup.body_nodes {
        body.append_child(body_node).unwrap();
    }

    Ok(document)
}

pub fn create_element_with_text(
    tag_name: &str,
    class_name: &str,
    text_content: Option<&str>,
) -> Result<web_sys::Element, Box<dyn error::Error>> {
    let element = create_element(tag_name)?;
    if class_name.len() > 0 {
        element.set_class_name(class_name);
    }
    element.set_text_content(text_content);

    Ok(element)
}

pub fn create_element_with_children(
    tag_name: &str,
    class_name: &str,
    child_elements: Vec<&Result<web_sys::Element, Box<dyn error::Error>>>,
) -> Result<web_sys::Element, Box<dyn error::Error>> {
    let element = create_element(tag_name)?;
    if class_name.len() > 0 {
        element.set_class_name(class_name);
    }

    for child_element in child_elements {
        //check if creation of the element is ok
        match child_element {
            Ok(child_element) => {
                //append_child and confirm if operation is successful
                match element.append_child(&web_sys::Node::from(child_element.to_owned())) {
                    Ok(_) => {}
                    Err(error) => return Err(format!("{:?}", error).into()),
                }
            }
            Err(error) => return Err(format!("{:?}", error).into()),
        }
    }

    Ok(element)
}
