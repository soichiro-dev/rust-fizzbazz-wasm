// wasm-bindgen を使用して Rust と JavaScript を協調させる
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;


// JavaScript が呼び出せる Rust 関数の作成
#[wasm_bindgen]
// TODO: 関数名が挨拶では動作がわからない
pub fn greet() -> String {
    // TODO: 文字列の連結をする必要があるか？配列など結果だけ返せばよいのでは
    let mut result: String = "".to_string();
    for n in 1..101 {
        if n % 15 == 0 {
            result.push_str("FizzBuzz");
        } else if n % 3 == 0 {
            result.push_str("Fizz");
        } else if n % 5 == 0 {
            result.push_str("Buzz");
        } else {
            result.push_str(&n.to_string());
        }
        result.push_str("\n");
    }

    return result;
}
