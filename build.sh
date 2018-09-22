#!/usr/bin

triple=wasm32-unknown-unknown
module=error_home

cargo +nightly build --target ${triple} && \
wasm-bindgen target/${triple}/debug/${module}.wasm --out-dir . && \
npm install && \
cp home/* . && \
npx webpack && \
cp jump/* . && \
npx webpack