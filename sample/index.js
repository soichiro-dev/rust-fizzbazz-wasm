// TODO: wasmファイルをimportするようにできないか
// npm installしたモジュールをimport
const fizzBuzzModule = import("./node_modules/@soichiro_dev/rust-fizzbazz-wasm/rust_fizzbazz_wasm.js");

// モジュール内の関数を実行しFizzBuzzゲームの結果を表示
fizzBuzzModule
.then(module => module.greet())
.then(result => document.write(result));
