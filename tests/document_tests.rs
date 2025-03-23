use std::vec;

use website_base::base_result::{BaseResult, ToBaseResult};
use website_base::document::{
    create_element_fn, create_element_ns, create_element_with_children,
    create_element_with_text, create_svg_element, get_element_by_id, initial_setup, HtmlNode,
    InitialSetup, ResultJs,
};
use website_base::navigation::is_navigation_supported;
use website_base::struct_node::StructNodeTrait;

use wasm_bindgen_test::*;
use web_sys::Event;
use website_base::window::get_window_location;

// wasm_bindgen_test_configure!(run_in_browser);

#[allow(dead_code)]
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

#[allow(dead_code)]
#[wasm_bindgen_test]
fn test_initial_setup_body_1_node() {
    struct Test1 {
        text_content: String,
    }

    impl StructNodeTrait for Test1 {
        fn render(&self) -> BaseResult<Vec<HtmlNode>> {
            Ok(vec![create_element_with_text(
                "div",
                Some(&self.text_content),
            )?])
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
        body_nodes: content_1,
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

#[allow(dead_code)]
#[wasm_bindgen_test]
fn get_element_by_id_1() {
    let node = get_element_by_id("");

    assert!(node.is_none());
}

#[allow(dead_code)]
#[wasm_bindgen_test]
fn get_element_by_id_2() {
    let node = create_element_with_text("div", None)
        .unwrap()
        .set_attribute("id", "get_element_by_id_2")
        .unwrap();

    // Document::new().unwrap().append_child(&node.to_node().unwrap()).unwrap();
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    body.append_child(&node.to_node().unwrap()).unwrap();

    let node = get_element_by_id("get_element_by_id_2");

    assert!(node.is_some());
}

#[allow(dead_code)]
#[wasm_bindgen_test]
fn create_element_with_text_error_1() {
    let node = create_element_with_text("", None);

    assert!(node.is_err());

    let error = node.unwrap_err();
    assert!(error.to_string().contains("InvalidCharacterError: Failed to execute 'createElement' on 'Document': The tag name provided ('') is not a valid name."));
}

#[allow(dead_code)]
#[wasm_bindgen_test]
fn create_element_with_text_1() {
    let node = create_element_with_text("span", Some("text"));

    assert!(node.is_ok());

    assert_eq!(
        "<span>text</span>",
        node.unwrap().to_element_node().unwrap().outer_html()
    );
}

#[allow(dead_code)]
#[wasm_bindgen_test]
fn create_element_with_children_error_1() {
    let node = create_element_with_children("", &vec![]);

    assert!(node.is_err());

    let error = node.unwrap_err();
    assert!(error.to_string().contains("InvalidCharacterError: Failed to execute 'createElement' on 'Document': The tag name provided ('') is not a valid name."));
}

#[allow(dead_code)]
#[wasm_bindgen_test]
fn create_element_fn_error_1() {
    let node = create_element_fn("div", |e| e.set_attribute("", ""));

    assert!(node.is_err());

    let error = node.unwrap_err();
    assert!(error.to_string().contains("InvalidCharacterError: Failed to execute 'setAttribute' on 'Element': '' is not a valid attribute name."));
}

#[allow(dead_code)]
#[wasm_bindgen_test]
fn set_attributes_error_1() {
    let node = create_element_fn("div", |e| e.set_attributes(vec![["att1", ""], ["", ""]]));

    assert!(node.is_err());

    let error = node.unwrap_err();
    assert!(error.to_string().contains("InvalidCharacterError: Failed to execute 'setAttribute' on 'Element': '' is not a valid attribute name."));
}

#[allow(dead_code)]
#[wasm_bindgen_test]
fn set_attributes_1() {
    let node = create_element_fn("div", |e| {
        e.set_attributes(vec![["att1", "value1"], ["att2", ""]])
    });

    assert!(node.is_ok());
    assert_eq!(
        "<div att1=\"value1\" att2=\"\"></div>",
        node.unwrap().to_element_node().unwrap().outer_html()
    );
}

#[allow(dead_code)]
#[wasm_bindgen_test]
fn create_element_fn_1() {
    let node = create_element_fn("div", |e| {
        e.set_attribute("class", "x")?
            .set_attribute("name", "value")
    });

    assert!(node.is_ok());
    assert_eq!(
        "<div class=\"x\" name=\"value\"></div>",
        node.unwrap().to_element_node().unwrap().outer_html()
    );
}

#[allow(dead_code)]
#[wasm_bindgen_test]
fn create_element_fn_2() {
    let node = create_element_fn("div", |e| {
        e.attach_shadow(true)?
            .append_children(&vec![create_element_with_text(
                "tag_name",
                Some("text_content"),
            )
            .unwrap()])
    });

    assert!(node.is_ok());
    assert_eq!(
        "<div></div>",
        node.unwrap().to_element_node().unwrap().outer_html()
    );
}

#[allow(dead_code)]
#[wasm_bindgen_test]
fn create_element_fn_3() {
    let node = create_element_fn("div", |e| {
        e.append_children(&vec![
            create_element_with_text("tag_name1", Some("text_content1"))
                .unwrap()
                .set_attribute("class", "class_name1")
                .unwrap(),
            create_element_with_text("tag_name2", Some("text_content2"))
                .unwrap()
                .set_attribute("class", "class_name2")
                .unwrap(),
            create_element_with_text("tag_name3", Some("text_content3"))
                .unwrap()
                .set_attribute("class", "class_name3")
                .unwrap(),
            create_element_with_text("tag_name4", Some("text_content4"))
                .unwrap()
                .set_attribute("class", "class_name4")
                .unwrap(),
        ])
    });

    assert!(node.is_ok());
    assert_eq!(
        "<div><tag_name1 class=\"class_name1\">text_content1</tag_name1><tag_name2 class=\"class_name2\">text_content2</tag_name2><tag_name3 class=\"class_name3\">text_content3</tag_name3><tag_name4 class=\"class_name4\">text_content4</tag_name4></div>",
        node.unwrap().to_element_node().unwrap().outer_html()
    );
}

#[allow(dead_code)]
#[wasm_bindgen_test]
fn create_element_with_children_1() {
    let node = create_element_with_children(
        "div",
        &vec![create_element_with_text("span", Some("some text"))
            .unwrap()
            .set_attribute("class", "class2")
            .unwrap()],
    )
    .unwrap()
    .set_attribute("class", "class1");

    assert!(node.is_ok());

    assert_eq!(
        "<div class=\"class1\"><span class=\"class2\">some text</span></div>",
        node.unwrap().to_element_node().unwrap().outer_html()
    );
}

#[allow(dead_code)]
#[wasm_bindgen_test]
fn to_result_js_1() {
    let node = create_element_with_children(
        "div",
        &vec![create_element_with_text("span", Some("some text")).unwrap()],
    );

    let new_node = node.to_result_js();

    assert!(new_node.is_ok());
}

#[allow(dead_code)]
#[wasm_bindgen_test]
fn to_result_js_error_1() {
    let node = create_element_with_children("div", &vec![]).unwrap();

    let att = node.set_attribute("", "value").to_result_js();

    assert!(att.is_err());

    let error = format!("{:?}", att.unwrap_err());
    assert!(error.contains("InvalidCharacterError: Failed to execute 'setAttribute' on 'Element': '' is not a valid attribute name."));
}

#[allow(dead_code)]
#[wasm_bindgen_test]
fn handle_js_error_1() {
    let node = create_element_with_text("div", None).unwrap();
    let node = node.to_element_node().unwrap();

    let error = node.set_attribute("", "").to_base_result();

    assert!(error.is_err());

    // assert!(error.unwrap_err().to_string().contains("InvalidCharacterError: Failed to execute 'setAttribute' on 'Element': '' is not a valid attribute name.\nError: Failed to execute 'setAttribute' on 'Element': '' is not a valid attribute name."));
    assert!(error.unwrap_err().to_string().contains("InvalidCharacterError: Failed to execute 'setAttribute' on 'Element': '' is not a valid attribute name."));
}

#[allow(dead_code)]
#[wasm_bindgen_test]
fn create_svg_1() {
    let svg = create_element_ns("http://www.w3.org/2000/svg", "svg").unwrap();

    let g = create_element_ns("http://www.w3.org/2000/svg", "g")
        .unwrap()
        .set_attributes(vec![[
            "style",
            "fill:lightgray;stroke:black;stroke-width:5;cursor:pointer;",
        ]])
        .unwrap();

    let rect = create_element_ns("http://www.w3.org/2000/svg", "rect").unwrap();
    rect.set_attributes(vec![
        ["x", "5"],
        ["y", "5"],
        ["rx", "10"],
        ["ry", "10"],
        ["width", "100"],
        ["height", "100"],
    ])
    .unwrap();

    g.append_children(&vec![rect]).unwrap();

    svg.set_attributes(vec![["width", "400"], ["height", "180"]])
        .unwrap();
    svg.append_children(&vec![g]).unwrap();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    body.append_child(&svg.to_node().unwrap()).unwrap();
}

#[allow(dead_code)]
#[wasm_bindgen_test]
fn create_svg_2() {
    let svg = create_svg_element("svg").unwrap();

    let circle = create_svg_element("circle")
        .unwrap()
        .set_attributes(vec![
            ["cx", "50"],
            ["cy", "50"],
            ["r", "40"],
            ["stroke", "green"],
            ["stroke-width", "4"],
            ["fill", "yellow"],
        ])
        .unwrap();

    svg.set_attributes(vec![["width", "100"], ["height", "100"]])
        .unwrap();
    svg.append_children(&vec![circle]).unwrap();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    body.append_child(&svg.to_node().unwrap()).unwrap();
}

#[allow(dead_code)]
#[wasm_bindgen_test]
fn get_document_location_1() {
    let location = get_window_location().unwrap();

    let path_name = location.pathname();

    assert!(path_name.is_ok());

    let path_name = path_name.unwrap();

    assert_eq!("/", path_name);
}

#[allow(dead_code)]
#[wasm_bindgen_test]
fn set_onnavigate_event_1() {
    let link = create_element_with_text("a", Some("text_content"))
        .unwrap()
        .set_attribute("href", "/test_this")
        .unwrap();
    // element.set_attribute("href", "/test_this").unwrap();

    let initial_setup_result = initial_setup(&InitialSetup {
        title: String::from("this is my new title"),
        head_nodes: vec![],
        body_nodes: vec![link.clone()],
    });

    assert!(initial_setup_result.is_ok());
    assert!(is_navigation_supported());

    website_base::navigation::set_onnavigate_event();

    let element = link.to_element_node().unwrap();

    element
        .dispatch_event(&Event::new("click").unwrap())
        .unwrap();
}
