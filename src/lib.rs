mod cleandoc;
mod docstrings;
mod extract;
mod generate;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_markdown(code: &str) -> String {
    generate::generate(&code)
}
