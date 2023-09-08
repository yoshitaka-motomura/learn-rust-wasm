use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use serde_wasm_bindgen::to_value;
mod datebook;
use datebook::calendar::holiday;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

#[wasm_bindgen]
pub fn holidays(year: i32) -> Result<JsValue, JsValue> {
    match holiday(year as u32) {
        Ok(holidays_data) => {
            match to_value(&holidays_data) {
                Ok(js_value) =>  Ok(js_value),
                Err(e) => {
                    error(&format!("Failed to serialize to JSON: {:?}", e));
                    Err(JsValue::NULL)
                }
            }
        }
        Err(e) => {
            error(&format!("Failed to get holidays: {:?}", e));
            Err(JsValue::NULL)
        }
    }
}
