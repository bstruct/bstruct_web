//! DOM manipulation and element creation utilities.
//!
//! This module provides a type-safe interface for creating and manipulating
//! HTML elements, documents, and document fragments.
//!
//! # Core Types
//!
//! - [`HtmlNode`]: An enum representing different node types (Element, Document, DocumentFragment)
//! - Helper functions for creating elements, setting attributes, and managing the DOM
//!
//! # Example
//!
//! ```no_run
//! use website_base::document::{create_element, HtmlNode};
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create an element with attributes
//! let div = create_element("div")?
//!     .set_attribute("class", "container")?
//!     .set_attribute("id", "main")?;
//!
//! // Create children and append
//! let children = vec![
//!     create_element("p")?,
//!     create_element("span")?,
//! ];
//! div.append_children(&children)?;
//! # Ok(())
//! # }
//! ```

use crate::{
    base_result::{BaseResult, ToBaseResult},
    error_messages::ErrorMessages,
};
use wasm_bindgen::prelude::*;

/// Represents different types of HTML/DOM nodes.
///
/// This enum provides a type-safe way to work with different node types
/// while maintaining their specific behaviors.
///
/// See: <https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType>
#[derive(Debug, Clone)]
pub enum HtmlNode {
    /// An HTML element node (e.g., div, span, p)
    ElementNode(web_sys::Element),
    // AttributeNode,
    // TextNode,
    // CDataSectionNode,
    // ProcessingInstructionNode,
    // CommentNode,
    /// The document node
    DocumentNode(web_sys::Document),
    // DocumentTypeNode,
    /// A document fragment (used for batch operations)
    DocumentFragmentNode(web_sys::DocumentFragment),
}

impl HtmlNode {
    /// Gets the current document as an HtmlNode.
    ///
    /// # Errors
    ///
    /// Returns an error if the document is not available.
    pub fn get_document() -> BaseResult<HtmlNode> {
        if let Ok(document) = get_document() {
            Ok(HtmlNode::DocumentNode(document))
        } else {
            Err(ErrorMessages::not_found("document").into())
        }
    }

    /// Converts this HtmlNode to a generic Node.
    ///
    /// # Errors
    ///
    /// This method shouldn't fail but returns BaseResult for consistency.
    pub fn to_node(&self) -> BaseResult<web_sys::Node> {
        match &self {
            HtmlNode::ElementNode(element) => Ok(web_sys::Node::from(element.to_owned())),
            HtmlNode::DocumentNode(document) => Ok(web_sys::Node::from(document.to_owned())),
            HtmlNode::DocumentFragmentNode(element) => Ok(web_sys::Node::from(element.to_owned())),
        }
    }

    /// Attempts to convert this HtmlNode to a Document.
    ///
    /// # Errors
    ///
    /// Returns an error if this node is not a DocumentNode.
    pub fn to_document_node(&self) -> BaseResult<web_sys::Document> {
        match &self {
            HtmlNode::DocumentNode(document) => Ok(document.to_owned()),
            _ => Err("node is not a document node".into()),
        }
    }

    /// Attempts to convert this HtmlNode to an Element.
    ///
    /// # Errors
    ///
    /// Returns an error if this node is not an ElementNode.
    pub fn to_element_node(&self) -> BaseResult<web_sys::Element> {
        match &self {
            HtmlNode::ElementNode(element) => Ok(element.to_owned()),
            _ => Err("node is not an element node".into()),
        }
    }

    /// Attaches a shadow root to this element.
    ///
    /// # Arguments
    ///
    /// * `open` - If true, creates an open shadow root; otherwise, creates a closed shadow root
    ///
    /// # Errors
    ///
    /// Returns an error if this is not an element node or if shadow DOM cannot be attached.
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

    /// Sets multiple attributes on this element.
    ///
    /// # Arguments
    ///
    /// * `attributes` - Vector of [name, value] pairs
    ///
    /// # Errors
    ///
    /// Returns an error if this is not an element node or if any attribute cannot be set.
    pub fn set_attributes(&self, attributes: Vec<[&str; 2]>) -> BaseResult<HtmlNode> {
        for attribute in attributes {
            self.set_attribute(attribute[0], attribute[1])?;
        }

        Ok(self.clone())
    }

    /// Appends multiple child nodes to this node and returns self for chaining.
    ///
    /// # Arguments
    ///
    /// * `child_elements` - Vector of child nodes to append
    ///
    /// # Errors
    ///
    /// Returns an error if any child cannot be appended.
    pub fn append_children(&self, child_elements: &Vec<HtmlNode>) -> BaseResult<HtmlNode> {
        append_children(self, child_elements)?;

        Ok(self.clone())
    }

    /// Adds an event listener to this node.
    ///
    /// # Arguments
    ///
    /// * `type_` - The event type (e.g., "click", "submit")
    /// * `f` - The callback function
    ///
    /// # Errors
    ///
    /// Returns an error if the event listener cannot be added.
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

    /// Adds a custom event listener to this node.
    ///
    /// # Arguments
    ///
    /// * `type_` - The custom event type
    /// * `f` - The callback function receiving CustomEvent
    ///
    /// # Errors
    ///
    /// Returns an error if the event listener cannot be added.
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

/// Creates a new HTML element with the specified tag name.
///
/// # Arguments
///
/// * `tag_name` - The HTML tag name (e.g., "div", "span", "p")
///
/// # Errors
///
/// Returns an error if the element cannot be created.
///
/// # Example
///
/// ```no_run
/// use website_base::document::create_element;
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let div = create_element("div")?;
/// let paragraph = create_element("p")?;
/// # Ok(())
/// # }
/// ```
pub fn create_element(tag_name: &str) -> BaseResult<HtmlNode> {
    let element = internal_create_element(tag_name).to_base_result()?;
    Ok(HtmlNode::ElementNode(element))
}

/// Creates an element with a namespace. Needed for SVG, MathML, and other XML-based elements.
///
/// # Arguments
///
/// * `namespace_uri` - The namespace URI (e.g., "http://www.w3.org/2000/svg")
/// * `tag_name` - The element tag name
///
/// # Errors
///
/// Returns an error if the element cannot be created.
///
/// See: <https://developer.mozilla.org/en-US/docs/Web/API/Document/createElementNS>
pub fn create_element_ns(namespace_uri: &str, tag_name: &str) -> BaseResult<HtmlNode> {
    let element = internal_create_element_ns(namespace_uri, tag_name).to_base_result()?;
    Ok(HtmlNode::ElementNode(element))
}

/// Creates an SVG element with the SVG namespace.
///
/// This is a convenience function that calls `create_element_ns` with
/// the SVG namespace URI ("http://www.w3.org/2000/svg").
///
/// # Arguments
///
/// * `tag_name` - The SVG element tag name (e.g., "svg", "path", "circle")
///
/// # Errors
///
/// Returns an error if the element cannot be created.
///
/// # Example
///
/// ```no_run
/// use website_base::document::create_svg_element;
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let svg = create_svg_element("svg")?;
/// let circle = create_svg_element("circle")?;
/// # Ok(())
/// # }
/// ```
pub fn create_svg_element(tag_name: &str) -> BaseResult<HtmlNode> {
    create_element_ns("http://www.w3.org/2000/svg", tag_name)
}

/// Gets an element from the document by its ID.
///
/// # Arguments
///
/// * `id` - The element's ID attribute value
///
/// # Returns
///
/// Returns `Some(HtmlNode)` if found, `None` otherwise.
pub fn get_element_by_id(id: &str) -> Option<HtmlNode> {
    let element = internal_get_element_by_id(id);
    if let Some(element) = element {
        Some(HtmlNode::ElementNode(element))
    } else {
        None
    }
}

/// Configuration for initializing a document's head and body.
pub struct InitialSetup {
    /// The document title
    pub title: String,
    /// Nodes to append to the document head (e.g., meta tags, stylesheets)
    pub head_nodes: Vec<HtmlNode>,
    /// Nodes to append to the document body
    pub body_nodes: Vec<HtmlNode>,
}

/// Initializes the document with title, head nodes, and body nodes.
///
/// # Arguments
///
/// * `setup` - Configuration containing title and nodes to add
///
/// # Errors
///
/// Returns an error if the document is not available or if nodes cannot be appended.
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

        for head_node in &setup.head_nodes {
            if let Ok(head_node) = head_node.to_node() {
                head.append_child(&head_node).unwrap();
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

/// Gets the current document object.
///
/// # Errors
///
/// Returns an error if the window or document is not available.
pub fn get_document() -> BaseResult<web_sys::Document> {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            return Ok(document);
        }
    }

    Err(ErrorMessages::not_found("document").into())
}

/// Creates an element with text content.
///
/// # Arguments
///
/// * `tag_name` - The HTML tag name
/// * `text_content` - Optional text content for the element
///
/// # Errors
///
/// Returns an error if the element cannot be created.
pub fn create_element_with_text(
    tag_name: &str,
    text_content: Option<&str>,
) -> BaseResult<HtmlNode> {
    let element = internal_create_element(tag_name).to_base_result()?;

    element.set_text_content(text_content);

    Ok(HtmlNode::ElementNode(element))
}

/// Creates an element and applies a configuration function to it.
///
/// # Arguments
///
/// * `tag_name` - The HTML tag name
/// * `element_fn` - A function that receives the element and returns it (possibly modified)
///
/// # Errors
///
/// Returns an error if the element cannot be created or if the function fails.
pub fn create_element_fn(
    tag_name: &str,
    element_fn: impl Fn(&HtmlNode) -> BaseResult<HtmlNode>,
) -> BaseResult<HtmlNode> {
    let html_node = create_element(tag_name)?;
    element_fn(&html_node)?;

    Ok(html_node)
}

/// Creates an element with child elements.
///
/// # Arguments
///
/// * `tag_name` - The HTML tag name
/// * `child_elements` - Vector of child nodes to append
///
/// # Errors
///
/// Returns an error if the element cannot be created or if children cannot be appended.
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
