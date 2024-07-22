## Rust-WASM
Notes - The code from the rust gets compiled to web assembly through `wasm-bindgen`. The website under www is used to test our code as wasm can't directly communicate to the javascript but js can read the contents of the wasm created from the rust project.

## To run:
Do `wasm-pack build` inside `game-of-life`, then run `npm run start` inside `www` and all the contents could be visible on `http://localhost:8080/`

This project was mainly done as to get my hands dirty with WASM. All the credit goes to https://rustwasm.github.io/docs/book/introduction.html
