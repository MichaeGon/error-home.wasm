const wasm = import('./error_home');

export function domain() {
    return document.domain;
}

export function random(n) {
    return Math.floor(Math.random() * n);
}

wasm.then(wasm => {
    var url = wasm.jump();
    location.href = url;
})