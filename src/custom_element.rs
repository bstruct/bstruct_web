use crate::base_result::{BaseResult, ToBaseResult};
use js_sys::eval;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

pub struct CustomElementDefinition {
    pub element_name: String,
    pub connected_callback_function_name: String,
}

pub fn define_custom_element(custom_element_definition: &CustomElementDefinition) -> BaseResult<()> {
    let name = &custom_element_definition.element_name;

    let class = format!("(class BstructCustomElement extends HTMLElement {{ constructor(){{super();}} connectedCallback(){{ {}(this); }} }})", custom_element_definition.connected_callback_function_name);
    let class = eval(&class).to_base_result()?;

    custom_elements_define(name, &class).to_base_result()
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = customElements, js_name = "define")]
    fn custom_elements_define(name: &str, constructor: &JsValue) -> Result<(), JsValue>;
}
