import * as wasm from "../../pkg/spicyrs";

const main = () => {
    wasm.set_up();

    window.addEventListener('ngspice_call', (event) => {
        fetch('http://134.122.41.6:80/ngspice', {
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
