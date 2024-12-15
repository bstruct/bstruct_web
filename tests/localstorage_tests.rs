use website_base::local_storage::{local_storage_get_item, local_storage_set_item};
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn local_storage_set_and_get_item_test() {
    let k = "k123";
    let v = "v123";

    local_storage_set_item(k, v);

    let value_from_storage = local_storage_get_item(k);

    assert!(value_from_storage.is_some());
    assert_eq!(v, value_from_storage.unwrap());
}

#[wasm_bindgen_test]
fn local_storage_get_item_test() {
    let k = "k1234";

    let value_from_storage = local_storage_get_item(k);

    assert!(value_from_storage.is_none());
}
