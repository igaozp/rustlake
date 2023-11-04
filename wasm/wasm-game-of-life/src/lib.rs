use wasm_bindgen::prelude::*;

mod utils;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {} to wasm-game-of-life!", name));
}
