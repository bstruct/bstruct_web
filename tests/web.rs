use wasm_bindgen_test::wasm_bindgen_test_configure;

wasm_bindgen_test_configure!(run_in_browser);

mod api_call_tests;
mod localstorage_tests;
mod document_tests;
use crate::api_call_tests::*;
use crate::document_tests::*;
use crate::localstorage_tests::*;
