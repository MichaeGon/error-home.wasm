const wasm = import('./error_home');

wasm.then(wasm => {
    wasm.greet("Rust");
})