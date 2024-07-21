use comrak::{markdown_to_html, Options};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn transform_from_md(val: &str) -> String {
    markdown_to_html(val, &Options::default())
}