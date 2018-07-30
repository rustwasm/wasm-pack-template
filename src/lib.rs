#![feature(use_extern_macros, wasm_import_module, wasm_custom_section)]

extern crate wasm_bindgen;

mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert(&format!("Hello, {{project-name}}!"));
}
