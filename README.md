# A Rust-written CPU path tracer for the web

## Introduction
The code is a Rust adaptation of the code found in the [Ray Tracing in One Weekend](https://raytracing.github.io/) book.

### Rationale
This project came about for three main reasons:
* I like Rust
* I'm curious about the state of WebAssembly
* I want to learn how path tracing works

This assignment is simply the perfect excuse to write more code in Rust while learning about something I'm interested in. 

### Goals
The primary goal of the project is to explore the viability of a WebAssembly based, computationally heavy project that runs entirely locally. This includes:
* Focus on performance
* Testing and CI - Tools that integrate well both with Rust and JavaScript
    * Debugging and benchmarking tools are of particular interest
* No reliance on Node.js

### Background
Rust is an ideal language as it natively supports WebAssembly building in the form of the `wasm32-unknown-unknown` target, without the need of tools such as [Emscripten](https://github.com/emscripten-core/emscripten).

The project uses `wasm-pack` and `wasm-bindgen` to compile Rust into WebAssembly and generate JavaScript bindings. Some JavaScript is still necessary for a few reasons:
* WebAssembly has no API access. For this project, this means any user input must go through JavaScript. This includes showing the image on screen or moving the camera in the scene.
* No DOM access. WebAssembly code has no knowledge of the page layout. JavaScript must be used to observe things such as the canvas size.
* Browsers require JavaScript. It's simply not possible to load WebAssembly without JavaScript.

For the above reasons, the code is roughly split so that JavaScript takes care of I/O while WebAssembly runs the path tracing engine.

Communication between JavaScript and WebAssembly is handled via shared memory to minimise overhead.

## Timeline
Current priority is to implement all of [the first book](https://raytracing.github.io/books/RayTracingInOneWeekend.html) in Rust. It is about 60% done so far.

Weeks are from the project start date.

| Estimated time | Project |
| -------------- | --------|
| Week 1 - 2 | Path tracing engine |
| Week 3 | Testing and CI set up, path tracing optimisations |
| Week 4 | Front end functionality - Camera movement, rendering options |
| Week 5 | Final report |

Stretch goals (if possible): Multi-threading, SIMD, second book of the series.
## Plan
Subject to change

This roughly represents the effort put into the project *besides the path tracing code*. I'm just doing that part for fun.
* 50% Researching WebAssembly capabilities
* 30% Integration with JavaScript
* 20% Overall usability - UI, path tracing accuracy (as is in the book)

## Building
### Prerequisites
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

## Libraries used as of now
* `wasm-bindgen` for interfacing with JavaScript
* `nalgebra` for linear algebra
* `fast-rand` for fast random number generation
