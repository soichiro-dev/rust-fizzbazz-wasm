const js = import("./node_modules/@soichiro_dev/rust-fizzbazz-wasm/rust_fizzbazz_wasm.js");
js.then(js => {
  js.greet("WebAssembly");
});