mod utils;

use wasm_bindgen::prelude::*;
pub use utils::set_panic_hook;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, {{project-name}}!");
}
