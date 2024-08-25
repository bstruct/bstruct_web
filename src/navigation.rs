use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(catch, js_namespace = navigation, js_name = "addEventListener")]
    fn navigation_add_event_listener(
        _type: &str,
        func: &web_sys::js_sys::Function,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = "NavigateEvent")]
    #[derive(Debug)]
    pub type NavigateEvent;

}

pub fn set_onnavigate_event() {
    // navigation_add_event_listener();

    let on_event_type_closure = Closure::wrap(Box::new(on_click) as Box<dyn Fn(&NavigateEvent)>);

    navigation_add_event_listener("navigate", on_event_type_closure.as_ref().unchecked_ref()).unwrap();

    on_event_type_closure.forget();
}

fn on_click(event: &NavigateEvent) {
    console::log_1(&JsValue::from_str(&format!("{:?}", event)));

    // let element = event
    //     .target()
    //     .unwrap()
    //     .dyn_into::<web_sys::Element>()
    //     .unwrap();

    // let mut custom_event_init = web_sys::CustomEventInit::new();
    // custom_event_init.bubbles(true);
    // custom_event_init.cancelable(true);
    // custom_event_init.composed(true);

    // let base_element = BaseElement::from_element(&element);
    // let type_ = match base_element.id().as_ref().unwrap().as_str() {
    //     BTN_FIRST_PAGE => EVENT_GO_TO_FIRST_PAGE,
    //     BTN_PREVIOUS_PAGE => EVENT_GO_TO_PREVIOUS_PAGE,
    //     BTN_NEXT_PAGE => EVENT_GO_TO_NEXT_PAGE,
    //     BTN_LAST_PAGE => EVENT_GO_TO_LAST_PAGE,
    //     _ => panic!("unknown button"),
    // };

    // let controls = element.closest(":host > [be_id=\"controls-background\"]");
    // if controls.is_ok() {
    //     if let Some(controls) = controls.unwrap() {
    //         if let Some(shadow) = controls.parent_node() {
    //             if let Some(st1) = shadow.last_child() {
    //                 st1.remove_child(&st1.last_child().unwrap()).unwrap();

    //                 let loading_div = &crate::createElement("div");
    //                 loading_div.set_text_content(Some("Loading..."));
    //                 st1.append_child(&loading_div).unwrap();

    //                 web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
    //                     "clear parent_element",
    //                 )));
    //             }
    //         }
    //     }
    // }

    // let action_event =
    //     web_sys::CustomEvent::new_with_event_init_dict(type_, &custom_event_init).unwrap();

    // element.dispatch_event(&action_event).unwrap();
}
