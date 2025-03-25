# This is all work-in-progress notes

# Introduction
This project implements the path tracing code from the book [Ray Tracing in One Weekend](https://raytracing.github.io/) in the Rust programming language. It is compiled to WebAssembly and runs in a web browser.

This report primarily serves to cover the topics of the presentation in more detail, and explore topics that were not included in the presentation.

# WebAssembly
## Design
WebAssembly "WASM" is a bytecode compilation target for high level languages, executed in a sandboxed virtual machine. It's a binary format, not designed to be written by humans. Its standardised human readable text representation is the Web Assembly Text Format "WAT".

WASM has the following high level goals
* Portability\
WASM isn't only for the web. It aims to be a generic bytecode format, independent of web browsers.
* Speed\
WASM aims to be small, quick to compile, and offer near-native runtime performance.
* Web integration\
WASM aims to be backwards compatible, and integrate well into the existing web ecosystem.

WASM is designed with a linear memory model. Addresses can be referenced by pointers in higher level code, transparent to the programmer. Memory must be allocated and freed manually, as there is no garbage collection. Multiple modules may be loaded simultaneously, each module is confined to its own `WebAssembly.Memory` object in a JavaScrip environment.

This project consists of a single WASM module, which contains the path tracer.

## JavaScript integration
WASM and JavaScript run separate from each other and interact through the WebAssembly JavaScript API. There are three primary ways of interaction:

* Import/Export functions\
WASM modules may import and export functions for consumption by JavaScript. Values passed through this abstraction are type converted.
* Shared memory\
The memory of every WASM module is visible to JavaScript through the `WebAssembly.Memory` object as an `ArrayBuffer`. Values may be directly read from and written to this buffer.
* `externref`\
WASM allows for direct consumption of JavaScript values through `externref` types. These may not be accessed or modified by the WASM code, they may only be passed in and out. This is primarily for interacting with other web APIs.

The code in this project uses a combination of the first two approaches. It is best explained by following the program flow.
1. The module expoes the following functions to JavaScript:
    * init()
    * init_settings()
    * trace()
    * get_texture()

These functions are only called by JavaScript.

2. `init()` creates a buffer for the scene and creates the `WebAssembly.Memory` object.
3. `init_settings()` creates a buffer in WASM memory to hold camera settings (resolution, samples per pixel (SPP), maximum recursion depth).\
The function returns a pointer to the location of this buffer in memory. Using this pointer, the JavaScript code writes the settings directly into the WASM memory buffer.
4. `trace()` runs the path tracer. It is a synchronous function, and only returns once the path tracer finishes.
5. `get_texture()` returns a pointer to the image output by the path tracer. This is used by JavaScript to index into the buffer and read the data.

Finally, the image data is loaded into WebGL as a texture and drawn onto a full-screen triangle.

# Rust
The primary language of this project is Rust. It is a statically typed, compiled systems programming language with a focus on performance and memory safety. It achieves this by use of the "Borrow checker", which prevents buffer overflows and validates the lifetimes of references to prevent use-after-free bugs. Other languages, notably C++ may also be written in a way to minimise the risk of such bugs.

The rich ecosystem and support for WASM makes Rust an ideal language for targetting this platform. This project uses the `wasm-bindgen` and `wasm-pack` crates to interface with JavaScript and help with compiling and packaging the WASM module.

> more todo here

# Graphics
The code in this project is a very basic path tracer. Path tracing simulates light reflecting off surfaces, passing through, and refracting inside objects. In a more abstract mathematical definition, path tracing solves this equation:

$L(v, w) = L_e(v, w) + \int_{\mathbb{S}^2} f_r(v, w', w) L(v', w') \cos \theta' , d\omega'$

This is called the "Rendering equation". Despite the name, it is not a full equation to solve all of rendering. It specifically covers two aspects:
* Emission
* Diffuse reflection

Notably it does not cover transmission of light. An intuitive breakdown of the equation is as follows:

The light traveling towards the viewer $w$ from point $v$ is equal to the light emitted from $v$ in direction of $w$, plus the total amount of light incoming to point $v$ and reflected in the direction $w$.

To solve the integral term, path tracing uses a *Monte Carlo method*. Monte Carlo methods can appxorimate any integral by taking random samples of the function in a given interval, averaging their value, and multiplying by the interavl's width. The equation is as follows:

$I \approx (b - a) \cdot \frac{1}{N} \sum_{i=1}^{N} f(x_i)$

Where\
$b$ is the end of the interval\
$a$ is the start of the interval\
$N$ is the number of samples taken\
$f$ is any function

The function converges as N approaches infinity. For this reason, Monte Carlo methods may approximate, but never equal the true integral of the function $f$.

Path tracing takes random samples by randomising the direction in which rays bounce upon hitting a surface. On a perfectly diffuse surface, rays may bounce in any direction within the unit hemisphere. As the surface gets "shinier", the cone in which rays may bounce narrows. On a perfectly smooth mirror surface, rays always bounce in the same direction.

> Fun fact: Many games use ray tracing only for reflections because of this property. Mirror surfaces only require a single ray per pixel, whereas a rough surface requires multiple. This greatly reduces the computational power required for path tracing.

There are various techniques to speed up path tracing. These include the use of a bounding volume hierarchy "BVH" to more efficiently store the scene, importance sampling to speed up convergeance of the Monte Carlo method, and ReSTIR GI to efficiently re-use already computed light paths. None of these methods are used in this path tracer.