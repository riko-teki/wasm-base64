extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn encode(plain_text: &str) {
    let bytes = plain_text.as_bytes();
    alert(&format!("Hello, {}!", &bytes.len()));
}

#[wasm_bindgen]
pub fn decode(encoded_text: &str){

}

