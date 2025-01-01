use crate::{base_result::BaseResult, document::HtmlNode};

pub trait StructNodeTrait {
    fn render(&self) -> Vec<BaseResult<HtmlNode>>;
}
