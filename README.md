# rise-wasm

## Explanation

The module here uses WebAssembly in two distinct ways. The first is the WebAssembly interpreter itself. It uses [`wasmi`](https://github.com/paritytech/wasmi) (an open source WebAssembly interpreter) to load the wasm binary and run a `main` function defined in the wasm. The lib (defined in `src/lib.rs`) loads defined memory, globals and imported functions into the scope of the wasm to be used. Imported functions can either be written in rust or in JavaScript (javascript imports can be found in `js/imports.js`) and get defined for the module in `src/import_globals.rs`.

WebAssembly here is also used to compile the rust so that it can be loaded as a normal javascript package. This uses [`wasm-pack`](https://github.com/rustwasm/wasm-pack) to compile and transform the rust, exposing a `verify` function that can be loaded with require (also creating TypeScript definitions).

The Test function here is just a little sample of a Pay-to-PubKeyHash (P2PKH) bitcoin contract. The wasm binary itself can be hashed and verified as the address. `hash160`, `compare` and `check_sig` are defined in `js/imports` and are used by the wasm to run the function. `test.js` / `test.wat` show an example of how this can be used.

## Building and Running

There are three languages here that are interacting JavaScript, rust and WebAssembly. To build:

* JavaScript: `yarn install`
* rust: `yarn build-rust` (uses `cargo` and [`wasm-pack`](https://github.com/rustwasm/wasm-pack) to output a js library)
* wasm: `yarn build-wasm` (uses `wat2wasm` from [`wabt`](https://github.com/WebAssembly/wabt))

A test JavaScript file named `test.js` shows an example of how to use the js lib and can be run with `yarn run-test`. All three can be done at once with `yarn all`

## Needs to be done

[ ] Fork `wasmi` to monitor opcodes and prevent too many instructions
[ ] Lib to hash wasm / create address and verify hash
[ ] MVP list of imported functions
[ ] Tests
[ ] Stack based memory using WebAssembly Tables instead of linear memory for better usability
[ ] Better / more consistent way to pass variables between JavaScript and rust functions
