# This is all work-in-progress notes

## Debugging
Debugging WASM is not fun. While there is break point support, it's not very useful for the following reasons:
* No debug symbols - function names are stripped from the code
* Call stack limit - WASM has incredibly strict limits on call stack size

Debug builds (which would include debug symbols) exceed the call stack limit and simply do not function. Release builds + debug maps work, but they include all optimisations and inline functions.

## Testing
Testing is also a sore point. `wasm-pack` includes a testing framework, but it's far from ideal. It runs into the same limits above, with missing debug symbols and a very strict call stack limit. Testing in Rust is done using assert_eq!(), which panics if the result does not match the expected value. This however does not work, as the panic!() exceeds the call stack limit and simply breaks instead.

## SIMD
This isn't fully done yet.

SIMD is one of the exclusive features of WebAssembly, not possible in JavaScript. Rust offers great SIMD support using the v128 type. The path tracer uses 4 32bit floats packed into one v128 variable to perform single-instruction vector component divide, addition, subtraction, and multiplication. As expected, debugging SIMD code is even harder than regular scalar maths, as Rust simply ignores things such as division by zero, where it would normally panic with scalar types.

One option is using LLVM's auto vectoriser, which is enabled by the `+simd128` compiler flag. This offers modest performance improvements.

To get maximum performance, I opted to write my own `vector3` library using SIMD intrinsics. This reduced render times by 25%.