//! Trait for rendering struct data as HTML nodes.
//!
//! This module provides a trait for implementing custom rendering logic
//! that converts Rust structs into HTML elements.
//!
//! # Example
//!
//! ```no_run
//! use website_base::struct_node::StructNodeTrait;
//! use website_base::document::{HtmlNode, create_element};
//! use website_base::base_result::BaseResult;
//!
//! struct MyComponent {
//!     title: String,
//! }
//!
//! impl HtmlNodeRender for MyComponent {
//!     fn render(&self) -> BaseResult<Vec<HtmlNode>> {
//!         let div = create_element("div")?;
//!         Ok(vec![div])
//!     }
//! }
//! ```

use crate::{base_result::BaseResult, document::HtmlNode};

/// Trait for types that can be rendered as HTML nodes.
///
/// Implement this trait to provide custom rendering logic for your types.
pub trait HtmlNodeRender {
    /// Renders this struct as a vector of HTML nodes.
    ///
    /// # Errors
    ///
    /// Returns an error if rendering fails.
    fn render(&self) -> BaseResult<Vec<HtmlNode>>;
}
