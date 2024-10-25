use crate::error_messages::ErrorMessages;
use wasm_bindgen::prelude::*;

//https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType
#[derive(Debug, Clone)]
pub enum HtmlNode {
    ElementNode(web_sys::Element),
    // AttributeNode,
    // TextNode,
    // CDataSectionNode,
    // ProcessingInstructionNode,
    // CommentNode,
    // DocumentNode,
    // DocumentTypeNode,
    DocumentFragmentNode(web_sys::DocumentFragment),
}

impl HtmlNode {
    pub fn to_node(&self) -> std::result::Result<web_sys::Node, Box<dyn error::Error>> {
        match &self {
            HtmlNode::ElementNode(element) => Ok(web_sys::Node::from(element.to_owned())),
            HtmlNode::DocumentFragmentNode(element) => Ok(web_sys::Node::from(element.to_owned())),
        }
    }

    pub fn to_element_node(&self) -> std::result::Result<web_sys::Element, Box<dyn error::Error>> {
        match &self {
            HtmlNode::ElementNode(element) => Ok(element.to_owned()),
            _ => Err("node is not an element node".into()),
        }
    }

    pub fn attach_shadow(
        &self,
        open: bool,
    ) -> std::result::Result<HtmlNode, Box<dyn error::Error>> {
        let shadow_init = if open {
            web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open)
        } else {
            web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Closed)
        };

        let element = self.to_element_node()?;
        let shadow = handle_js_error(element.attach_shadow(&shadow_init))?;

        Ok(HtmlNode::DocumentFragmentNode(shadow.into()))
    }

    #[doc = "Set attribute of ElementNode's. If HtmlNode doesn't support set attribute, the function will return an error."]
    #[doc = "Returns the same element to facilitate the functional programming pattern."]
    pub fn set_attribute(
        &self,
        name: &str,
        value: &str,
    ) -> Result<HtmlNode, Box<dyn error::Error>> {
        let element = self.to_element_node()?;
        handle_js_error(element.set_attribute(name, value))?;

        Ok(self.clone())
    }

    pub fn set_attributes(
        &self,
        attributes: Vec<[&str; 2]>,
    ) -> Result<HtmlNode, Box<dyn error::Error>> {
        for attribute in attributes {
            self.set_attribute(attribute[0], attribute[1])?;
        }

        Ok(self.clone())
    }

    #[doc = "Append children to node and return the original node"]
    pub fn append_children(
        &self,
        child_elements: Vec<&Result<HtmlNode, Box<dyn error::Error>>>,
    ) -> Result<HtmlNode, Box<dyn error::Error>> {
        append_children(self, &child_elements)?;

        Ok(self.clone())
    }
}

pub trait ResultJs<T: std::clone::Clone> {
    fn to_result_js(&self) -> Result<T, JsError>;
}

impl<T> ResultJs<T> for std::result::Result<T, Box<dyn error::Error>>
where
    T: std::clone::Clone + std::fmt::Debug,
{
    fn to_result_js(&self) -> Result<T, JsError> {
        if self.is_ok() {
            Ok(self.as_ref().unwrap().clone())
        } else {
            Err(JsError::new(&self.as_ref().unwrap_err().to_string()))
        }
    }
}

use std::error;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = document, js_name = "createElement")]
    fn internal_create_element(tag_name: &str) -> Result<web_sys::Element, JsValue>;

    #[wasm_bindgen(js_namespace = document, js_name = "getElementById")]
    fn internal_get_element_by_id(id: &str) -> Option<web_sys::Element>;
}

pub fn create_element(tag_name: &str) -> Result<HtmlNode, Box<dyn error::Error>> {
    let element = handle_js_error(internal_create_element(tag_name))?;
    Ok(HtmlNode::ElementNode(element))
}

pub fn get_element_by_id(id: &str) -> Option<HtmlNode> {
    let element = internal_get_element_by_id(id);
    if let Some(element) = element {
        Some(HtmlNode::ElementNode(element))
    } else {
        None
    }
}

pub struct InitialSetup {
    pub title: String,
    pub head_nodes: Vec<HtmlNode>,
    pub body_nodes: Vec<HtmlNode>,
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
        body.append_child(&body_node.to_node()?).unwrap();
    }

    Ok(document)
}

pub fn create_element_with_text(
    tag_name: &str,
    class_name: &str,
    text_content: Option<&str>,
) -> Result<HtmlNode, Box<dyn error::Error>> {
    let element = handle_js_error(internal_create_element(tag_name))?;

    if class_name.len() > 0 {
        element.set_class_name(class_name);
    }
    element.set_text_content(text_content);

    Ok(HtmlNode::ElementNode(element))
}

pub fn create_element_fn(
    tag_name: &str,
    element_fn: impl Fn(&HtmlNode) -> Result<HtmlNode, Box<dyn error::Error>>,
) -> Result<HtmlNode, Box<dyn error::Error>> {
    let html_node = create_element(tag_name)?;
    element_fn(&html_node)?;

    Ok(html_node)
}

pub fn create_element_with_children(
    tag_name: &str,
    class_name: &str,
    child_elements: Vec<&Result<HtmlNode, Box<dyn error::Error>>>,
) -> Result<HtmlNode, Box<dyn error::Error>> {
    let element = create_element(tag_name)?;
    if class_name.len() > 0 {
        element.set_attribute("class", class_name)?;
    }

    append_children(&element, &child_elements)?;

    Ok(element)
}

fn append_children(
    element: &HtmlNode,
    child_elements: &Vec<&Result<HtmlNode, Box<dyn error::Error>>>,
) -> Result<(), Box<dyn error::Error>> {
    let node = element.to_node()?;

    for child_element in child_elements {
        //check if creation of the element is ok
        match child_element {
            Ok(child_element) => {
                //append_child and confirm if operation is successful
                match &child_element.to_node() {
                    Ok(child) => {
                        handle_js_error(node.append_child(child))?;
                    }
                    Err(error) => return Err(format!("{:?}", error).into()),
                }
            }
            Err(error) => return Err(format!("{:?}", error).into()),
        }
    }

    Ok(())
}

pub fn handle_js_error<T>(result: Result<T, JsValue>) -> Result<T, Box<dyn error::Error>> {
    match result {
        Ok(e) => Ok(e),
        Err(error) => Err(format!("{:?}", error).into()),
    }
}
