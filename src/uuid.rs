use js_sys::JsString;

use crate::base_result::{BaseResult, ToBaseResult};

//Math.random().toString(36).substring(2) + (new Date()).getTime().toString(36)
#[doc = "Generate unique identifier"]
#[doc = "result of the function: Math.random().toString(36).substring(2) + (new Date()).getTime().toString(36)"]
pub fn generate_uuid() -> BaseResult<String> {
    let r = js_sys::Number::to_string(&js_sys::Number::from(js_sys::Math::random()), 36)
        .to_base_result()?;

    let r = r.substr(2, r.length() as i32 - 2).pad_end(12, "0");

    let t = js_sys::Number::to_string(&js_sys::Number::from(js_sys::Date::new_0().get_time()), 36)
        .to_base_result()?;

    Ok(r.concat(&t).as_string().unwrap_or_default())
}
