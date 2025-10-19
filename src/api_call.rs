//! HTTP API call utilities using the browser's Fetch API.
//!
//! This module provides a high-level interface for making HTTP requests
//! from WebAssembly using the browser's Fetch API.
//!
//! # Example
//!
//! ```rust
//! use website_base::api_call::ApiCallRequest;
//! use wasm_bindgen::JsValue;
//!
//! async fn fetch_data() {
//!     let request = ApiCallRequest::new(
//!         "https://api.example.com/data",
//!         "GET",
//!         vec![["Content-Type", "application/json"]],
//!         None,
//!     );
//!     
//!     let (response, body) = request
//!         .make_api_call_resolve_json_body()
//!         .await
//!         .unwrap();
//! }
//! ```

use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

use crate::base_result::{BaseResult, ToBaseResult};

/// Represents an HTTP API request with method, headers, and optional body.
pub struct ApiCallRequest<'a> {
    url: &'a str,
    method: &'a str,
    headers: Vec<[&'a str; 2]>, // mode: opts.mode(web_sys::RequestMode::NoCors);
    body: Option<&'a JsValue>,
}

impl<'b> ApiCallRequest<'b> {
    /// Creates a new API call request.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to make the request to
    /// * `method` - HTTP method (GET, POST, PUT, DELETE, etc.)
    /// * `headers` - Vector of header key-value pairs
    /// * `body` - Optional request body as JsValue
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

    /// Makes an API call using the browser's fetch function.
    ///
    /// Returns the Response object on success.
    ///
    /// # Errors
    ///
    /// Returns an error if the fetch fails or if the window is not available.
    pub async fn make_api_call(&self) -> BaseResult<web_sys::Response> {
        let request_init = web_sys::RequestInit::new();
        request_init.set_method(self.method);
        // request_init.set_mode(web_sys::RequestMode::Cors);

        let headers = web_sys::Headers::new().to_base_result()?;
        for header in &self.headers {
            headers.set(header[0], header[1]).to_base_result()?;
        }
        request_init.set_headers(&headers);

        if let Some(body) = self.body {
            request_init.set_body(body);
        }

        let request = web_sys::Request::new_with_str_and_init(
            self.url,
            &request_init,
        ).to_base_result()?;

        if let Some(window) = web_sys::window() {
            let resp_value = JsFuture::from(window.fetch_with_request(&request)).await;
            let resp_value = resp_value.to_base_result()?;

            Ok(web_sys::Response::from(resp_value))
        } else {
            Err("window not found".into())
        }
    }

    /// Makes an API call and resolves the response body as JSON.
    ///
    /// Returns a tuple of (Response, parsed JSON body).
    ///
    /// # Errors
    ///
    /// Returns an error if the fetch fails or if JSON parsing fails.
    pub async fn make_api_call_resolve_json_body(
        &self,
    ) -> BaseResult<(web_sys::Response, JsValue)> {
        let response = self.make_api_call().await?;
        let body = wasm_bindgen_futures::JsFuture::from(response.json().unwrap()).await;
        let body = body.to_base_result()?;

        Ok((response, body))
    }
}
