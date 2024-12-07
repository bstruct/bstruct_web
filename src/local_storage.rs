use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = "localStorage.setItem")]
    pub fn local_storage_set_item(key: &str, value: &str);

    #[wasm_bindgen(js_namespace = window, js_name = "localStorage.getItem")]
    pub fn local_storage_get_item(key: &str) -> Option<String>;
}