use wasm_bindgen_test::wasm_bindgen_test_configure;

wasm_bindgen_test_configure!(run_in_browser);

mod api_call_tests;
mod localstorage_tests;
mod document_tests;
#[allow(unused_imports)]
use crate::api_call_tests::*;
#[allow(unused_imports)]
use crate::document_tests::*;
#[allow(unused_imports)]
use crate::localstorage_tests::*;
