use serde::Deserialize;
use website_base::api_call::ApiCallRequest;
use wasm_bindgen_test::wasm_bindgen_test;
use web_sys::Blob;

#[allow(dead_code)]
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

#[derive(Debug, Deserialize)]
pub struct TestStruct {
    #[allow(dead_code)]
    pub id: i32,
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub company: String,
}

#[allow(dead_code)]
#[wasm_bindgen_test]
async fn make_api_call_resolve_json_body_1() {
    let (response, body) = ApiCallRequest::new(
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
    .make_api_call_resolve_json_body()
    .await
    .unwrap();

    assert_eq!(response.status(), 200);

    // assert!(body.is_object().as_string().unwrap().starts_with("["));
    assert!(body.is_object());
    assert!(body.is_array());

    // try to parse the body as json
    let body = serde_wasm_bindgen::from_value::<Vec<TestStruct>>(body);

    assert!(body.is_ok());
    assert_eq!(body.unwrap().len(), 10);

}