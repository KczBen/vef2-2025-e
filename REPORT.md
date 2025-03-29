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
    * add_sphere()

These functions are only called by JavaScript.

2. `init()` creates a buffer for the scene and creates the `WebAssembly.Memory` object.
3. `init_settings()` creates a buffer in WASM memory to hold camera settings.\
The function returns a pointer to the location of this buffer in memory. Using this pointer, the JavaScript code writes the settings directly into the WASM memory buffer.
4. `add_sphere()` adds a sphere to the scene. Since JavaScript doesn't have the required SIMD type, it takes individual floating point arguments which are then packed into a `v128` type in Rust.
5. `trace()` runs the path tracer. It is a synchronous function, and only returns once the path tracer finishes.
6. `get_texture()` returns a pointer to the image output by the path tracer. This is used by JavaScript to index into the buffer and read the data.

Finally, the image data is loaded into WebGL as a texture and drawn onto a full-screen triangle.

From here on, the Settings struct exposes 3 additional members used for fast communication between Rust and JavaScript. These members are observed and modified by both parts of the code. Designing such an interface requires some thought, as there is no mutual exclusion mechanism in place.

The exposed members and their purposes are:
1. `texture_changed`\
This is a flag set by Rust to notify JavaScript that a new texture is ready for use. Upon this flag being set, JavaScript loads the new texture into the WebGL buffer and resets the flag to await a new texture again.
2. `settings_changed`\
This is a flag set by JavaScript to inform Rust that the old scene settings are no longer valid, and path tracing should be stopped. The state of this flag is checked before a new sample is rendered. If it is set, the loop breaks.
3. `busy`\
This is a flag set by Rust to inform JavaScript that the path tracer is currently busy. It serves to synchronise the submission of new camera data to the path tracer. When the settings change, the `settings_changed` flag is set to 1. From here, it takes some time for the path tracer to stop, depending on how far along the current sample is. JavaScript waits for the `busy` flag to be set to 0 before submitting the new camera settings to Rust and restarting the path tracer with the new settings.

# Rust
The primary language of this project is Rust. It is a statically typed, compiled systems programming language with a focus on performance and memory safety. It achieves this by use of the "Borrow checker", which validates the lifetimes of references at compile time to prevent use-after-free bugs, dangling pointers and other memory safety violations. Array accesses are checked at runtime to prevent buffer overflow errors. Other languages, notably C++ may also be written in a way to minimise the risk of such bugs.

The rich ecosystem and support for WASM makes Rust an ideal language for targetting this platform. This project uses the `wasm-bindgen` and `wasm-pack` crates to interface with JavaScript and help with compiling and packaging the WASM module.

## Error handling
Rust has no `null` value, and no exceptions. Instead, it uses the `Option<T>` and `Result<T, E>` return types for error handling.

### Option<T>
An option is a value that may or may not exists. In the code, it is used for ray sphere intersection. Options can have one of two values:
`Some<T>`\
`Some<T>` indicates the presence of a value of type `T`.

`None`\
`None` is the safe equivalent of `null`, and indicates the lack of a value.

The use of these types ensures that the return value of a function is always valid. Both cases must be handled. The function `unwrap()` may be used to neglect handling the `None` case. If `unwrap()` consumes a `None`, it panics and the program safely crashes.

The modern C++ equivalent type is `std::optional<T>` *as of C++17*

### Result<T, E>
Results are the type equivalent of exceptions in other languages. Results can have one of two values:
`T`\
`T` is the successful return value of the function of type `T`.

`E`\
`E` is the error type of the function, indicating that the function failed. It replaces C++ exceptions or C error codes.

As with `Option<T>`, both cases must be handled. `unwrap()` may be used to neglect handling the `E` case. If `unwrap()` consumes an `E`, it panics and the program safely crashes.

The modern C++ equivalent type is `std::expected<T, E>` *as of C++23*

## Memory management
## Ownership
The Rust borrow checker enforces ownership rules. Every value must have an owner, and the value's lifetime depends on the lifetime of the owner. When the owner goes out of scope, its lifetime ends and all owned values are dropped. Values may be borrowed from their owner by use of a reference `&`. References are immutable by default, their values may only be read but not written. Multiple immutable references to a value may exist at once. Mutable references `&mut` allow the underlying value to be changed. Only one mutable reference to a value may exist at one time.

Since ownership isn't transferred when a value is borrowed, the borrowed value cannot be dropped moved or dropped.

Ownership may be transferred when values are explicitly moved. When ownership is transferred, references to the original owner become invalid.

This mechanism effectively prevents dangling pointers and use-after-free errors. Since this all happens in the compiler, it has no effect on runtime performance.

## Heap allocations
All Rust primitive types are stack allocated by default. Heap allocations can be made in sevaral ways. The simples way is by use of the `Box<T>` type. `Box` creates a heap allocation and stores a pointer to it. It is the equivalent of `malloc` in C. Unlike C, the allocated memory is freed when the `Box` goes out of scope. `Box` uniquely owns the reference to the heap allocation.

`Box` allocations may be leaked using the `leak()` function. This allows the heap allocation to outlive its owner. As long as the new reference is accessible, a new `Box` can be created from it and properly dropped when it is no longer needed. If the reference goes out of scope, the heap allocation is not automatically freed, resulting in a memory leak.

Other heap allocators are `Rc<T>` and `Arc<T>`. Like `Box`, these create a heap allocation and store a pointer to it. Unlike `Box`, `Rc` and `Arc` are reference counted shared pointers. There may be multiple references to the heap allocation. Their main differenec is that `Arc` is an atomic thread safe type.

`Arc` is used for storing scene and object data in this project. This would allow multiple objects to reference the same material, though in practice, it is not used that way.

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

The function converges as $N$ approaches infinity. For this reason, Monte Carlo methods may approximate, but never equal the true integral of the function $f$.

Path tracing takes random samples by randomising the direction in which rays bounce upon hitting a surface. On a perfectly diffuse surface, rays may bounce in any direction within the unit hemisphere. As the surface gets "shinier", the cone in which rays may bounce narrows. On a perfectly smooth mirror surface, rays always bounce in the same direction.

> Many games use ray tracing only for reflections because of this property. Mirror surfaces only require a single ray per pixel, whereas a rough surface requires multiple. This greatly reduces the computational power required for path tracing.

There are various techniques to speed up path tracing. These include the use of a bounding volume hierarchy "BVH" to more efficiently store the scene, importance sampling to speed up convergeance of the Monte Carlo method, and ReSTIR GI to efficiently re-use already computed light paths. None of these methods are used in this path tracer.

## Linear algebra and the `v128` type
WASM provides the `v128` primitive type for vector operations. As part of WASM's goal to offer near-native performance, this type is designed to take advantage of Single Instruction Multiple Data "SIMD" extensions in CPU instruction sets. These are `SSE` on x86 and `NEON` on ARM, or equivalent on other architectures. Note that the WASM specification does not require *any* specific instructions. The implementation may choose to emulate these instructions if the hardware doesn't support it.

This code uses SIMD instructions to perform fast vector operations for addition, division, subtraction, cross and dot product operations. `v128` types are not constrained to any data format, their interpretation depends solely on the instruction using the type. To effectively store ray origins and directions, the `f32x4` type is used, allowing for 4 dimensional vectors. Since the $w$ component is never used, it is set to zero at all times to not interfere with other operations.

The use of the `v128` type and associated SIMD instructions results in a 25% reduction in render time for the test scene. Further uses for SIMD in this project include per-pixel gamma correction, resulting in another 1-2% reduction in render time. Vectors are also randomly generated using a SIMD implementation of the xorshift algorithm.