use std::error;
use crate::document::HtmlNode;

pub trait StructNodeTrait {
    fn render(&self) -> Result<HtmlNode, Box<dyn error::Error>>;
}
