mod convert;
mod errors;
mod qase;
mod utils;

use wasm_bindgen::prelude::*;

pub fn markdown_to_qase(
    suite_header: &str,
    markdown_text: &str,
) -> Result<String, errors::ConvertError> {
    let suite = convert::convert_md_to_qase(suite_header, markdown_text)?;
    match serde_json::to_string_pretty(&suite) {
        Ok(serialized) => Ok(serialized),
        Err(err) => Err(errors::ConvertError::JSONSerializationError(err)),
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn web_convert_markdown_to_qase(suite_header: &str, markdown_text: &str) -> String {
    match markdown_to_qase(suite_header, markdown_text) {
        Ok(result) => result,
        Err(_err) => String::from("error converting to qase json"),
    }
}
