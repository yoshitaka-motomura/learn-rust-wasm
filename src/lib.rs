use wasm_bindgen::prelude::*;
mod datebook;
use datebook::calendar::OutputFormat;
use datebook::calendar::holiday;

#[wasm_bindgen]
pub fn holidays(year: i32, format: &str)-> Option<String> {
    let f = match format {
        "json" => OutputFormat::JSON,
        "csv" => OutputFormat::CSV,
        "yaml" => OutputFormat::YAML,
        _ => OutputFormat::JSON,
    };
    holiday(f, year as u32).ok()
}
