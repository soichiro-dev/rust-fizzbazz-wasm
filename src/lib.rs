// wasm-bindgen を使用して Rust と JavaScript を協調させる
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;


// JavaScript 内の外部関数を Rust から呼び出す
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}


// JavaScript が呼び出せる Rust 関数の作成
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
