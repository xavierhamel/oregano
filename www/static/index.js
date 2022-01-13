import * as wasm from "wasm-spicyrs";

const main = () => {
    wasm.set_up();

    window.addEventListener('ngspice_call', (event) => {
        fetch('http://localhost:3000/api/run', {
            method: 'POST',
            mode: 'cors',
            body: event.detail,
        })
            .then((response) => response.text())
            .then((value) => {
                window.dispatchEvent(new CustomEvent('ngspice_response', {
                    detail: value,
                }))
            });
    });
};
main();
