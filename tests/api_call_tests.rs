use bstruct_browser_base::api_call::ApiCallRequest;
use wasm_bindgen_test::wasm_bindgen_test;
use web_sys::Blob;

#[wasm_bindgen_test]
async fn make_api_call_get_1() {
    let response = ApiCallRequest::new(
        "https://fake-json-api.mock.beeceptor.com/users",
        "GET",
        vec![
            ["Accept", "*/*"],
            //https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Allow-Origin
            ["Access-Control-Allow-Origin", "*"],
            ["Accept-Encoding", "gzip, deflate, br"],
            // ["Accept-Language", "en-GB,en;q=0.9"],
        ],
        None,
    )
    .make_api_call()
    .await;

    assert!(response.is_ok());

    let response = response.unwrap();

    assert_eq!(response.status(), 200);
    assert_eq!(response.status_text(), "");
    assert!(!response.redirected());

    let body = wasm_bindgen_futures::JsFuture::from(response.blob().unwrap())
        .await
        .unwrap();
    let body = Blob::from(body);

    assert_eq!(body.type_(), "application/json");
    assert!(body.size() > 0.0);

    let body = wasm_bindgen_futures::JsFuture::from(body.text())
        .await
        .unwrap();

    assert!(body.as_string().unwrap().starts_with("["));
}
