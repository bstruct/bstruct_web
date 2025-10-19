# bstruct_website_base

[![Tests](https://github.com/bstruct/website_base/actions/workflows/browser_tests.yml/badge.svg)](https://github.com/bstruct/website_base/actions/workflows/browser_tests.yml)
[![Chrome](https://img.shields.io/badge/tested%20on-Chrome-4285F4?logo=googlechrome&logoColor=white)](https://github.com/bstruct/website_base/actions/workflows/browser_tests.yml)
[![Firefox](https://img.shields.io/badge/tested%20on-Firefox-FF7139?logo=firefox&logoColor=white)](https://github.com/bstruct/website_base/actions/workflows/browser_tests.yml)
[![Safari](https://img.shields.io/badge/tested%20on-Safari-006CFF?logo=safari&logoColor=white)](https://github.com/bstruct/website_base/actions/workflows/browser_tests.yml)

A Rust/WebAssembly library providing high-level, type-safe abstractions for web browser APIs. Build modern web applications with Rust, leveraging the power of WebAssembly.

## Features

- **Type-safe DOM manipulation** - Create and manage HTML elements with a fluent API
- **Custom Elements** - Define and register Web Components easily
- **HTTP API calls** - Make fetch requests with automatic JSON handling
- **Local Storage** - Simple key-value storage interface
- **Navigation API** - Handle browser navigation events (experimental)
- **Window utilities** - Access window and location objects

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
website_base = { git = "https://github.com/bstruct/website_base" }
```

## Quick Start

```rust
use website_base::document::{create_element, HtmlNode};
use website_base::base_result::BaseResult;

fn main() -> BaseResult<()> {
    // Get the document
    let document = HtmlNode::get_document()?.to_document_node()?;
    let body = document.body().unwrap();
    
    // Create an element with attributes
    let div = create_element("div")?
        .set_attribute("class", "container")?
        .set_attribute("id", "main")?;
    
    // Add event listener
    div.set_event_listener("click", |_event| {
        web_sys::console::log_1(&"Clicked!".into());
    })?;
    
    // Append to body
    body.append_child(&div.to_node()?)?;
    
    Ok(())
}
```

## API Documentation

### `document` - DOM Manipulation

The core module for creating and managing HTML elements.

#### Creating Elements

```rust
use website_base::document::{create_element, create_svg_element, create_element_with_text};

// Create a basic element
let div = create_element("div")?;

// Create with text content
let paragraph = create_element_with_text("p", Some("Hello, World!"))?;

// Create SVG elements
let svg = create_svg_element("svg")?;
let circle = create_svg_element("circle")?;
```

#### Setting Attributes

```rust
// Single attribute
element.set_attribute("class", "btn btn-primary")?;

// Multiple attributes
element.set_attributes(vec![
    ["class", "container"],
    ["id", "main"],
    ["data-value", "42"],
])?;
```

#### Managing Children

```rust
// Create children and append
let children = vec![
    create_element("h1")?,
    create_element("p")?,
    create_element("div")?,
];
element.append_children(&children)?;

// Or create element with children
let container = create_element_with_children("div", &children)?;
```

#### Event Listeners

```rust
// Standard events
element.set_event_listener("click", |event| {
    web_sys::console::log_1(&"Clicked!".into());
})?;

// Custom events
element.set_custom_event_listener("my-event", |event| {
    let detail = event.detail();
    // Handle custom event
})?;
```

#### Shadow DOM

```rust
// Attach shadow root (open mode)
let shadow = element.attach_shadow(true)?;

// Add content to shadow DOM
let style = create_element("style")?;
shadow.to_node()?.append_child(&style.to_node()?)?;
```

### `api_call` - HTTP Requests

Make HTTP requests using the Fetch API.

```rust
use website_base::api_call::ApiCallRequest;
use wasm_bindgen::JsValue;

async fn fetch_data() -> BaseResult<JsValue> {
    let request = ApiCallRequest::new(
        "https://api.example.com/data",
        "GET",
        vec![
            ["Content-Type", "application/json"],
            ["Authorization", "Bearer token123"],
        ],
        None,
    );
    
    // Get response and parsed JSON
    let (response, body) = request
        .make_api_call_resolve_json_body()
        .await?;
    
    Ok(body)
}
```

#### POST Request with Body

```rust
use wasm_bindgen::JsValue;
use js_sys::JSON;

async fn post_data() -> BaseResult<()> {
    let data = JsValue::from_str(r#"{"name": "Alice", "age": 30}"#);
    
    let request = ApiCallRequest::new(
        "https://api.example.com/users",
        "POST",
        vec![["Content-Type", "application/json"]],
        Some(&data),
    );
    
    let response = request.make_api_call().await?;
    Ok(())
}
```

### `custom_element` - Web Components

Define and register custom HTML elements.

```rust
use website_base::custom_element::{define_custom_element, CustomElementDefinition};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init_my_element(element: &web_sys::Element) {
    element.set_inner_html("<h1>Hello from custom element!</h1>");
}

// Define the custom element
let definition = CustomElementDefinition {
    element_name: "my-element".to_string(),
    connected_callback_function_name: "init_my_element".to_string(),
};

define_custom_element(&definition)?;

// Now use it in your document
let element = create_element("my-element")?;
document.body()?.append_child(&element.to_node()?)?;
```

### `local_storage` - Persistent Storage

Simple interface for browser localStorage.

```rust
use website_base::local_storage::{local_storage_set_item, local_storage_get_item};

// Store data
local_storage_set_item("user_id", "12345");
local_storage_set_item("theme", "dark");

// Retrieve data
if let Some(user_id) = local_storage_get_item("user_id") {
    println!("User ID: {}", user_id);
}

// Handle missing keys
match local_storage_get_item("nonexistent") {
    Some(value) => println!("Found: {}", value),
    None => println!("Key not found"),
}
```

### `navigation` - Navigation API

Handle browser navigation (experimental API).

```rust
use website_base::navigation::{is_navigation_supported, set_onnavigate_event};

if is_navigation_supported() {
    set_onnavigate_event();
} else {
    // Fallback for unsupported browsers
    web_sys::console::log_1(&"Navigation API not supported".into());
}
```

### `window` - Window Utilities

Access window and location objects.

```rust
use website_base::window::get_window_location;

let location = get_window_location()?;

// Get current URL
let href = location.href()?;
let pathname = location.pathname()?;
let search = location.search()?;

// Navigate
location.set_href("https://example.com")?;
```

### `base_result` - Error Handling

Common result type used throughout the library.

```rust
use website_base::base_result::{BaseResult, ToBaseResult};

fn my_function() -> BaseResult<String> {
    let result = some_operation().to_base_result()?;
    Ok(result)
}
```

### `struct_node` - Component Pattern

Trait for rendering structs as HTML.

```rust
use website_base::struct_node::StructNodeTrait;
use website_base::document::{HtmlNode, create_element};
use website_base::base_result::BaseResult;

struct UserCard {
    name: String,
    email: String,
}

impl StructNodeTrait for UserCard {
    fn render(&self) -> BaseResult<Vec<HtmlNode>> {
        let card = create_element("div")?
            .set_attribute("class", "user-card")?;
        
        let name = create_element_with_text("h2", Some(&self.name))?;
        let email = create_element_with_text("p", Some(&self.email))?;
        
        card.append_children(&vec![name, email])?;
        
        Ok(vec![card])
    }
}
```

## Complete Example

```rust
use website_base::document::{create_element, create_element_with_text, HtmlNode, initial_setup, InitialSetup};
use website_base::base_result::BaseResult;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> BaseResult<()> {
    // Initialize document
    let setup = InitialSetup {
        title: "My App".to_string(),
        head_nodes: vec![],
        body_nodes: vec![],
    };
    initial_setup(&setup)?;
    
    // Create app structure
    let header = create_element("header")?;
    let title = create_element_with_text("h1", Some("Welcome!"))?;
    header.append_children(&vec![title])?;
    
    let main = create_element("main")?;
    let button = create_element("button")?
        .set_attribute("class", "btn")?;
    button.set_event_listener("click", |_| {
        web_sys::console::log_1(&"Button clicked!".into());
    })?;
    
    main.append_children(&vec![button])?;
    
    // Add to document
    let document = HtmlNode::get_document()?.to_document_node()?;
    let body = document.body().unwrap();
    body.append_child(&header.to_node()?)?;
    body.append_child(&main.to_node()?)?;
    
    Ok(())
}
```

## Browser Compatibility

Tested on:
- ✅ Chrome/Chromium (latest)
- ✅ Firefox (latest)
- ✅ Safari (latest)

## Building and Testing

```bash
# Run local test file
sh compile_and_test.sh
```

## License

See [LICENSE](LICENSE) file for details.

## References

- [Rust WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [wasm-bindgen Documentation](https://rustwasm.github.io/wasm-bindgen/)
- [MDN Web Docs](https://developer.mozilla.org/)



