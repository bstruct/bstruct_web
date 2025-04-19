use crate::{
    base_result::{BaseResult, ToBaseResult},
    error_messages::ErrorMessages,
};
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
    DocumentNode(web_sys::Document),
    // DocumentTypeNode,
    DocumentFragmentNode(web_sys::DocumentFragment),
}

impl HtmlNode {
    pub fn get_document() -> BaseResult<HtmlNode> {
        if let Ok(document) = get_document() {
            Ok(HtmlNode::DocumentNode(document))
        } else {
            Err(ErrorMessages::not_found("document").into())
        }
    }

    pub fn to_node(&self) -> BaseResult<web_sys::Node> {
        match &self {
            HtmlNode::ElementNode(element) => Ok(web_sys::Node::from(element.to_owned())),
            HtmlNode::DocumentNode(document) => Ok(web_sys::Node::from(document.to_owned())),
            HtmlNode::DocumentFragmentNode(element) => Ok(web_sys::Node::from(element.to_owned())),
        }
    }

    pub fn to_document_node(&self) -> BaseResult<web_sys::Document> {
        match &self {
            HtmlNode::DocumentNode(document) => Ok(document.to_owned()),
            _ => Err("node is not a document node".into()),
        }
    }

    pub fn to_element_node(&self) -> BaseResult<web_sys::Element> {
        match &self {
            HtmlNode::ElementNode(element) => Ok(element.to_owned()),
            _ => Err("node is not an element node".into()),
        }
    }

    pub fn attach_shadow(&self, open: bool) -> BaseResult<HtmlNode> {
        let shadow_init = if open {
            web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open)
        } else {
            web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Closed)
        };

        let element = self.to_element_node()?;
        let shadow = element.attach_shadow(&shadow_init).to_base_result()?;

        Ok(HtmlNode::DocumentFragmentNode(shadow.into()))
    }

    #[doc = "Set attribute of ElementNode's. If HtmlNode doesn't support set attribute, the function will return an error."]
    #[doc = "Returns the same element to facilitate the functional programming pattern."]
    pub fn set_attribute(&self, name: &str, value: &str) -> BaseResult<HtmlNode> {
        let element = self.to_element_node()?;
        element.set_attribute(name, value).to_base_result()?;

        Ok(self.clone())
    }

    pub fn set_attributes(&self, attributes: Vec<[&str; 2]>) -> BaseResult<HtmlNode> {
        for attribute in attributes {
            self.set_attribute(attribute[0], attribute[1])?;
        }

        Ok(self.clone())
    }

    #[doc = "Append children to node and return the original node"]
    pub fn append_children(&self, child_elements: &Vec<HtmlNode>) -> BaseResult<HtmlNode> {
        append_children(self, child_elements)?;

        Ok(self.clone())
    }

    pub fn set_event_listener<T>(&self, type_: &str, f: T) -> BaseResult<HtmlNode>
    where
        T: Fn(&web_sys::Event),
        T: 'static,
    {
        let on_event_type_closure: Closure<dyn Fn(&web_sys::Event)> =
            Closure::wrap(Box::new(f) as Box<dyn Fn(&web_sys::Event)>);

        self.to_node()?
            .add_event_listener_with_callback(type_, on_event_type_closure.as_ref().unchecked_ref())
            .to_base_result()?;

        on_event_type_closure.forget();

        Ok(self.clone())
    }

    pub fn set_custom_event_listener<T>(&self, type_: &str, f: T) -> BaseResult<HtmlNode>
    where
        T: Fn(&web_sys::CustomEvent),
        T: 'static,
    {
        let on_event_type_closure: Closure<dyn Fn(&web_sys::CustomEvent)> =
            Closure::wrap(Box::new(f) as Box<dyn Fn(&web_sys::CustomEvent)>);

        self.to_node()?
            .add_event_listener_with_callback(type_, on_event_type_closure.as_ref().unchecked_ref())
            .to_base_result()?;

        on_event_type_closure.forget();

        Ok(self.clone())
    }

    pub fn remove_event_listener<T>(&self, type_: &str, f: T) -> BaseResult<HtmlNode>
    where
        T: Fn(&web_sys::Event),
        T: 'static,
    {
        let on_event_type_closure: Closure<dyn Fn(&web_sys::Event)> =
            Closure::wrap(Box::new(f) as Box<dyn Fn(&web_sys::Event)>);

        self.to_node()?
            .remove_event_listener_with_callback(
                type_,
                on_event_type_closure.as_ref().unchecked_ref(),
            )
            .to_base_result()?;

        on_event_type_closure.forget();

        Ok(self.clone())
    }

    pub fn remove_custom_event_listener<T>(&self, type_: &str, f: T) -> BaseResult<HtmlNode>
    where
        T: Fn(&web_sys::CustomEvent),
        T: 'static,
    {
        let on_event_type_closure: Closure<dyn Fn(&web_sys::CustomEvent)> =
            Closure::wrap(Box::new(f) as Box<dyn Fn(&web_sys::CustomEvent)>);

        self.to_node()?
            .remove_event_listener_with_callback(
                type_,
                on_event_type_closure.as_ref().unchecked_ref(),
            )
            .to_base_result()?;

        on_event_type_closure.forget();

        Ok(self.clone())
    }
}

pub trait ResultJs<T: std::clone::Clone> {
    fn to_result_js(&self) -> Result<T, JsError>;
}

impl<T> ResultJs<T> for std::result::Result<T, Box<dyn std::error::Error>>
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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = document, js_name = "createElement")]
    fn internal_create_element(tag_name: &str) -> Result<web_sys::Element, JsValue>;

    #[wasm_bindgen(catch, js_namespace = document, js_name = "createElementNS")]
    fn internal_create_element_ns(
        namespace_uri: &str,
        tag_name: &str,
    ) -> Result<web_sys::Element, JsValue>;

    #[wasm_bindgen(js_namespace = document, js_name = "getElementById")]
    fn internal_get_element_by_id(id: &str) -> Option<web_sys::Element>;
}

pub fn create_element(tag_name: &str) -> BaseResult<HtmlNode> {
    let element = internal_create_element(tag_name).to_base_result()?;
    Ok(HtmlNode::ElementNode(element))
}

#[doc = "Creates an element with a namespace. Needed for SVG's HTML and MathML elements."]
#[doc = "https://developer.mozilla.org/en-US/docs/Web/API/Document/createElementNS#namespaceuri"]
pub fn create_element_ns(namespace_uri: &str, tag_name: &str) -> BaseResult<HtmlNode> {
    let element = internal_create_element_ns(namespace_uri, tag_name).to_base_result()?;
    Ok(HtmlNode::ElementNode(element))
}

#[doc = "Sugar sintax on createElementNS - https://developer.mozilla.org/en-US/docs/Web/API/Document/createElementNS to comply to SVG needs"]
#[doc = "Creates an element of a given tag with the namespace 'http://www.w3.org/2000/svg'"]
pub fn create_svg_element(tag_name: &str) -> BaseResult<HtmlNode> {
    create_element_ns("http://www.w3.org/2000/svg", tag_name)
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

pub fn initial_setup(setup: &InitialSetup) -> BaseResult<web_sys::Document> {
    if let Ok(document) = get_document() {
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
            if let Ok(body_node) = body_node.to_node() {
                body.append_child(&body_node).unwrap();
            }
        }

        Ok(document)
    } else {
        Err(ErrorMessages::not_found("document").into())
    }
}

pub fn get_document() -> BaseResult<web_sys::Document> {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            return Ok(document);
        }
    }

    Err(ErrorMessages::not_found("document").into())
}

pub fn create_element_with_text(
    tag_name: &str,
    text_content: Option<&str>,
) -> BaseResult<HtmlNode> {
    let element = internal_create_element(tag_name).to_base_result()?;

    element.set_text_content(text_content);

    Ok(HtmlNode::ElementNode(element))
}

pub fn create_element_fn(
    tag_name: &str,
    element_fn: impl Fn(&HtmlNode) -> BaseResult<HtmlNode>,
) -> BaseResult<HtmlNode> {
    let html_node = create_element(tag_name)?;
    element_fn(&html_node)?;

    Ok(html_node)
}

pub fn create_element_with_children(
    tag_name: &str,
    child_elements: &Vec<HtmlNode>,
) -> BaseResult<HtmlNode> {
    let element = create_element(tag_name)?;
    append_children(&element, child_elements)?;

    Ok(element)
}

fn append_children(element: &HtmlNode, child_elements: &Vec<HtmlNode>) -> BaseResult<()> {
    let node = element.to_node()?;

    for child_element in child_elements {
        node.append_child(&child_element.to_node()?)
            .to_base_result()?;
    }

    Ok(())
}
