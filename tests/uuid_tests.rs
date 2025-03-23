use wasm_bindgen_test::wasm_bindgen_test;
use website_base::uuid::generate_uuid;

#[allow(dead_code)]
#[wasm_bindgen_test]
fn test_initial_setup_title() {
    let uuid = generate_uuid();

    assert!(uuid.is_ok());

    let uuid = uuid.unwrap();

    assert_eq!(uuid.len(), 20);
    assert!(!uuid.contains(" "));
}
