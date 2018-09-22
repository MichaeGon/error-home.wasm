const wasm = import('./error_home');

export function domain() {
    return document.domain;
}

export function random(n) {
    return Math.floor(Math.random() * n);
}

wasm.then(wasm => {
    var pageContent = wasm.home(document.domain);

    document.title = pageContent.title();

    var elem = document.createElement("div");
    elem.innerHTML = pageContent.body();
    document.body.appendChild(elem);

})