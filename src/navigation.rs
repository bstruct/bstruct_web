use wasm_bindgen::prelude::*;
use web_sys::{console, Event, Url};

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(catch, js_namespace = navigation, js_name = "addEventListener")]
    fn navigation_add_event_listener(
        _type: &str,
        func: &web_sys::js_sys::Function,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(extends = Event, js_name = "NavigateEvent")]
    #[derive(Debug)]
    pub type NavigateEvent;

    #[wasm_bindgen(extends = Event, js_name = "NavigationDestination")]
    #[derive(Debug)]
    pub type NavigationDestination;

    // https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/intercept

    #[wasm_bindgen(getter, method, js_class = "NavigateEvent", js_name = "canIntercept")]
    pub fn can_intercept(this: &NavigateEvent) -> bool;

    // #[wasm_bindgen(structural, method, js_name = "hashChange")]
    // pub fn hash_change(this: &NavigateEvent) -> bool;

    #[wasm_bindgen(structural, method, js_class = "NavigateEvent", js_name = "intercept")]
    pub fn intercept(this: &NavigateEvent);

    #[wasm_bindgen(structural, method, getter, js_class = "NavigateEvent", js_name = "destination")]
    pub fn destination(this: &NavigateEvent) -> NavigationDestination;

    #[wasm_bindgen(structural, method, getter, js_class = "NavigationDestination", js_name = "url")]
    pub fn url(this: &NavigationDestination) -> Url;

}

pub fn is_navigation_supported() -> bool {
    let evaluation = js_sys::eval("window.navigation !== undefined");
    if let Ok(evaluation) = evaluation {
        if let Some(result) = evaluation.as_bool() {
            return result;
        }
    }

    false
}

pub fn set_onnavigate_event() {
    let on_event_type_closure = Closure::wrap(Box::new(on_navigate) as Box<dyn Fn(&NavigateEvent)>);

    navigation_add_event_listener("navigate", on_event_type_closure.as_ref().unchecked_ref())
        .unwrap();

    on_event_type_closure.forget();
}

fn on_navigate(event: &NavigateEvent) {
    console::log_1(&JsValue::from_str(&format!("{:?}", event)));

    if event.can_intercept() {
        console::log_1(&JsValue::from_str(&format!(
            "---- destination url: {:?}",
            event.destination().url()
        )));

        event.intercept();
    }
}
