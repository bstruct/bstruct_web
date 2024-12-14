use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

use crate::{base_result::BaseResult, document::handle_js_error};

pub struct ApiCallRequest<'a> {
    url: &'a str,
    method: &'a str,
    headers: Vec<[&'a str; 2]>, // mode: opts.mode(web_sys::RequestMode::NoCors);
    body: Option<&'a JsValue>,
}

impl<'b> ApiCallRequest<'b> {
    pub fn new(
        url: &'b str,
        method: &'b str,
        headers: Vec<[&'b str; 2]>,
        body: Option<&'b JsValue>,
    ) -> ApiCallRequest<'b> {
        ApiCallRequest {
            url: url,
            method: method,
            headers: headers,
            body: body,
        }
    }

    #[doc = "Make an API call using the fetch browser function"]
    pub async fn make_api_call(&self) -> BaseResult<web_sys::Response> {
        let request_init = web_sys::RequestInit::new();
        request_init.set_method(self.method);
        // request_init.set_mode(web_sys::RequestMode::Cors);

        let headers = handle_js_error(web_sys::Headers::new())?;
        for header in &self.headers {
            handle_js_error(headers.set(header[0], header[1]))?;
        }
        request_init.set_headers(&headers);

        if let Some(body) = self.body {
            request_init.set_body(body);
        }

        let request = handle_js_error(web_sys::Request::new_with_str_and_init(
            self.url,
            &request_init,
        ))?;

        if let Some(window) = web_sys::window() {
            let resp_value = JsFuture::from(window.fetch_with_request(&request)).await;
            let resp_value = handle_js_error(resp_value)?;

            Ok(web_sys::Response::from(resp_value))
        } else {
            Err("window not found".into())
        }
    }

    pub async fn make_api_call_resolve_body(
        &self,
    ) -> BaseResult<(web_sys::Response, JsValue)> {
        let response = self.make_api_call().await?;
        let body = wasm_bindgen_futures::JsFuture::from(response.blob().unwrap()).await;
        let body = handle_js_error(body)?;
        let body = web_sys::Blob::from(body);
        let body = handle_js_error(wasm_bindgen_futures::JsFuture::from(body.text()).await)?;

        Ok((response, body))
    }
}
