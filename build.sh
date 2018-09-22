#!/usr/bin

triple=wasm32-unknown-unknown
module=error_home

cargo +nightly build --target ${triple}
wasm-bindgen target/${triple}/debug/${module}.wasm --out-dir .
npx webpack