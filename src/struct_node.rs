use std::error;

pub trait StructNodeTrait {
    fn render(&self) -> Result<web_sys::Node, Box<dyn error::Error>>;
}
