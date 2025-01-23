# A Rust-written CPU path tracer for the web

The code is a Rust adaptation of the code found in the [Ray Tracing in One Weekend](https://raytracing.github.io/) book.

## Goal
The goal of the project is primarily to explore options outside of JavaScript for the web. Rust is an ideal language as it natively supports WebAssembly building in the form of the `wasm32-unknown-unknown` target, without the need of tools such as [Emscripten](https://github.com/emscripten-core/emscripten).

Efforts have been made to minimize JavaScript usage and avoid platform dependencies. As a result, there are no JavaScript frameworks used in the project, and all of the custom JavaScript code is contained in a single file. The auto-generated JavaScript files for interfacing with WebAssembly can be found in the `/pkg` directory.

## Building
### Build dependencies
Ensure you have the following dependencies installed:
* [Rustup](https://www.rust-lang.org/tools/install)
* [wasm-pack](https://github.com/rustwasm/wasm-bindgen)

### Building the project
* Clone the GitHub repo:\
`git clone https://github.com/KczBen/vef2-2025-e`

* Navigate to the project directory:\
`cd vef2-2025-e`

* Build with `wasm-pack`:\
`wasm-pack build --target web`

## Libraries used
* `wasm-bindgen` for interfacing with JavaScript
* `nalgebra` for linear algebra
* `fast-rand` for fast random number generation
* `rayon` for simple multi-threading