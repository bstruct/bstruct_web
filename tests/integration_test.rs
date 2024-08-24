use bstruct_browser_base::document::{
    create_element_with_children, create_element_with_text, initial_setup, HtmlNode, InitialSetup,
};
use bstruct_browser_base::struct_node::StructNodeTrait;

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_initial_setup_title() {
    let initial_setup_result = initial_setup(&InitialSetup {
        title: String::from("this is my new title"),
        head_nodes: vec![],
        body_nodes: vec![],
    });

    assert!(initial_setup_result.is_ok());

    let document = initial_setup_result.unwrap();
    assert!(document.head().is_some());
    assert_eq!(document.head().unwrap().outer_html(), "<head>\n    <meta content=\"text/html;charset=utf-8\" http-equiv=\"Content-Type\">\n  <title>this is my new title</title></head>");
}

#[wasm_bindgen_test]
fn test_initial_setup_body_1_node() {
    struct Test1 {
        text_content: String,
    }

    impl StructNodeTrait for Test1 {
        fn render(&self) -> Result<HtmlNode, Box<dyn std::error::Error>> {
            create_element_with_text("div", "", Some(&self.text_content))
        }
    }

    let content_1 = Test1 {
        text_content: String::from("my content is amazing"),
    }
    .render()
    .unwrap();

    let initial_setup_result = initial_setup(&InitialSetup {
        title: String::from("this is my new title"),
        head_nodes: vec![],
        body_nodes: vec![content_1],
    });

    assert!(initial_setup_result.is_ok());

    let document = initial_setup_result.unwrap();
    assert!(document.body().is_some());
    assert!(document
        .body()
        .unwrap()
        .outer_html()
        .contains("<div>my content is amazing</div>"));
}

#[wasm_bindgen_test]
fn create_element_with_text_error_1() {
    let node = create_element_with_text("", "", None);

    assert!(node.is_err());

    let error = node.unwrap_err();
    assert!(error.to_string().contains("InvalidCharacterError: Failed to execute 'createElement' on 'Document': The tag name provided ('') is not a valid name."));
}

#[wasm_bindgen_test]
fn create_element_with_text_1() {
    let node = create_element_with_text("span", "", Some("text"));

    assert!(node.is_ok());

    assert_eq!("<span>text</span>", node.unwrap().to_element_node().unwrap().outer_html());
}

#[wasm_bindgen_test]
fn create_element_with_children_error_1() {
    let node = create_element_with_children("", "", vec![]);

    assert!(node.is_err());

    let error = node.unwrap_err();
    assert!(error.to_string().contains("InvalidCharacterError: Failed to execute 'createElement' on 'Document': The tag name provided ('') is not a valid name."));
}

#[wasm_bindgen_test]
fn create_element_with_children_error_2() {
    let node = create_element_with_children(
        "div",
        "",
        vec![&create_element_with_text("", "", Some("child_elements"))],
    );

    assert!(node.is_err());

    let error = node.unwrap_err();
    assert!(error.to_string().contains("InvalidCharacterError: Failed to execute 'createElement' on 'Document': The tag name provided ('') is not a valid name."));
}

#[wasm_bindgen_test]
fn create_element_with_children_1() {
    let node = create_element_with_children(
        "div",
        "class1",
        vec![&create_element_with_text(
            "span",
            "class2",
            Some("some text"),
        )],
    );

    assert!(node.is_ok());

    assert_eq!(
        "<div class=\"class1\"><span class=\"class2\">some text</span></div>",
        node.unwrap().to_element_node().unwrap().outer_html()
    );
}
