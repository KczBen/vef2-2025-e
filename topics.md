# This contains the presentation topics and some notes
## What I made
For my project, I wrote a path tracer (or ray tracer). It is based on the book `Ray Tracing in One Weekend`. It's a very simple path tracer with only bounce lighting. It doesn't look particularly interesting, but it's not the focus of the presentation. You don't need knowledge about graphics to follow along, but I will mention the basics of ray and path tracing:

Ray tracing is a very simple concept: You draw a line in 3D space and see where it lands. You only need a starting point and a direction.

Path tracing is the graphics technique to simulate light reflecting off surfaces in the world. At its core, it uses recursive ray tracing.

While the graphics part is interesting, the focus is on this:
> show JavaScript code

This is all the JavaScript code. There's a bit of logic, some WebGL shader code, but no path tracer. That all happens in these two lines of code:

```
async function runWasm() {
    wasmMemory = (await init()).memory;
    ...
    trace();
    ...
}
```

All of the path tracing code is written in WebAssembly.

## What is WASM
WebAssembly the "second language" of the web, expanding - *but not replacing* - JavaScript. It's a statically typed language and is supported by all modern browsers. It aims to run and compile faster than JavaScript, benchmarks put it at around 10% to 30% faster than equivalent JavaScript code.

Now that all sounds good, but there are a few limitations:

JavaScript is mandatory. In order to load WebAssembly in a web environment, some JavaScript code must be present. All API access must go through JavaScript.

Second, WebAssembly looks like *this:*


```
 (func $func35 (result i32)
    (local $var0 i32) (local $var1 i32) (local $var2 i32) (local $var3 i32) (local $var4 i32) (local $var5 i32) (local $var6 externref)
    global.get $global0
    i32.const -64
    i32.add
    local.tee $var2
    global.set $global0
    block $label2
      block $label1
        block $label0
        ...)
```

WebAssembly has the following primitive data types:
```
Integer:
i32
i64

Floating point:
f32
f64

Vector:
v128
```

The other important data type is `externref`. This type allows WebAssembly code to directly receive and handle JavaScript values.

As the name suggest, this is an assembly-like language. While you can write it by hand, the intended way to use it is to write code in a higher level langauge, and then compile it into WASM. This has two primary benefits:
* Portable code\
You can mostly re-use the core of your desktop application on the web, without needing to re-write anything other than the UI
* It isn't JavaScript\
`[] == ![]; // true`

### Language considerations
[There are a number of languages you may choose from](https://github.com/appcypher/awesome-wasm-langs), but in practice, you will most likely end up using one of these three:
C, C++, Rust. If you choose to write your code in C, keep security in mind. All of the C vulnerabilities you know (buffer overflow, use-after-free, double free, etc.) carry over to WASM! The Web Assembly VM runs in a sandbox, but it does not protect you from your own mistakes.

I chose to write my project in Rust. Security isn't a concern for a path tracer, I simply like the language. It also offers a mature Web Assembly ecosystem for use in your project.

### Hello, Rust!
Let's do a "Hello world" in Rust, and see how to run it in a browser

First, "Hello world" in Rust is very simple:

```
//lib.rs
fn main() {
  println!("Hello, world!");
}
```

The syntax borrows mainly from C++ and OCaml.

Other example:
```
//lib.rs
fn main() {
  let a = 1;
  let b = 3;

  let c = add_two(a, b);
}

fn add_two(a: i64, b: i64) -> i64 {
  return a + b;
}
```

### Into the browser
To get it running in a browser, we'll use two extra dependencies: `wasm-pack` and `wasm-bindgen`.

`wasm-pack` packages your `wasm` code and glue logic into an ES module for use in your main JavaScript code.\
`wasm-bindgen` is used for importing JavaScript functions and exporting Rust functions.

First, we need to decide *how* we want to print "Hello, world". As mentioned before, we have no *native* I/O from WebAssembly, so `println!()` will not work. We have two options:

 * Pass the string to JavaScript and print from there
 * Import `console.log()` and call it from Rust

Both approaches are valid, but the former is better for portability. JavaScript imports introduce platform dependency, which might not be ideal for a shared code base.

We need a function that returns a string in Rust:
```
fn print_string() -> String {
  return "Hello, world!".to_string();
}
```

Now, we need to make it visible to JavaScript. We'll do that using the `wasm-bindgen` crate.
```
#[wasm_bindgen]
pub fn print_string() -> String {
  return "Hello, world!".to_string();
}
```

`#[wasm_bindgen]` is an attribute that adds metadata to the function. It informs the compiler about certain things, in this case, it exposes the function in the final WASM module. We also need to make the function public for it to work.

Now we're all set up on the Rust side, we only need to compile with\
`wasm-pack build --target web`

This creates an ES module under `pkg/`, which we can import into our `index.js`:
```
import init, { print_string } from './pkg/vef2_2025_e.js';
```

We're almost there. We need to initialise the WASM module, and call our string function. Here is the full code:

```
import init, { print_string } from './pkg/vef2_2025_e.js';

async function main() {
  await init();
  console.log(print_string());
}

main()
```

We can't run this in Node yet, it doesn't have full support for WebAssembly. Instead, we'll need to run it in a browser.

> put link to local web server running it here

And there it is!

## Performance
Path tracing is a very slow process, so any performance uplift is welcome. 

First, we'll measure where we are, and define some goals. To measure, we can use the developer tools in our browser to profile our code. We'll need source mappings (use a debug build!) to get function names.

> show devtools pane first without mappings, then with mappings

Now we can see what takes the most time, and where we need to focus.

So, we need goals. *Run it until it's done* is good, but because of mathematical reasons (see full report), path tracing takes infinite time to complete. That's quite hard to fit in a 10 minute presentation. We need to limit the recursion depth and the samples per pixel (again, see report for details if interested) as I've already done.

Instead, we'll just say *Make it faster*. Not the best goal, but it is easy to achieve.

We gained some just by using WebAssembly instead of JavaScript. Compiler optimisations help as well, but we can do more.

Here we'll use the special `v128` type, and a bit of linear algebra. Rays consist of two parts: an origin point, and a direction. These are all 3 dimensional points in space - vectors. We use a lot of vector operations - addition, subtraction, multiplication, division, cross and dot products - to trace our rays.

Luckily, there is special hardware in modern CPUs to speed up these operations. If you want to make use of that hardware on the web, you need to use WebAsembly. The `v128` type does exactly that.

Normally when you add two vectors, it takes 3 operations: Add the x coordinates, add the y coordinates, and finally add the z coordinates.

x<sub>1</sub> + x<sub>2</sub>\
y<sub>1</sub> + y<sub>2</sub>\
z<sub>1</sub> + z<sub>2</sub>

In Rust, it would look like this:
```
fn add_vec(vec1: vector3, vec2: vector3) -> vector3 {
  return vector3::new(vec1.x + vec2.x,
                      vec1.y + vec2.y,
                      vec1.z + vec2.z);
}
```

With the `v128` type, we can use a programming paradigm called `SIMD` (Single Instruction Multiple Data) to turn this into a single operation. `v128` can be defined as any one of these types:
```
i8x16
u8x16
i32x4
u32x4
i64x2
u64x2

f32x4
f64x2
```

For this, we want to use the `f32x4` type. That is, 4 32-bit floating point numbers packed into one CPU register. Using this, we can now write our vector addition function as such:
```
fn add_vec(vec1: v128, vec2: v128) -> v128 {
  return f32x4_add(vec1, vec2);
}
```

This performs the exact same operation as the first function, but in a single instruction instead of 3. Rewriting the path tracer to use this approach results in a 25% reduction in runtime.

You don't need to rewrite your code to take advantage of this exclusive feature: Adding the `+simd128` compiler flag to Rust enables the auto vectoriser, which will recognise some SIMD patterns and automatically insert these instructions.

## Quirks and issues
The process isn't flawless, and there are a few points that stand out:

* Debugging is hard. Really hard.

The console in your browser doesn't tell you *Anything*. Get used to seeing the ominuous error message `Unreachable executed`. Something went wrong... but what? Even if you can figure it out with breakpoints, your function names are stripped from the WASM module. You can try using source mappings, but the compiler doesn't always output it even when you tell it to. Good luck!

* Testing is fragile.

Debugging is hard, and so is testing. It runs into one big problem: The Stack. Web Assembly has a very limited call stack size, and you will often get the error `Too much recursion` even if you don't use any. This wouldn't be an issue, if not for Rust's design. The Rust compiler very aggressively inlines functions in `release` mode. However, testing is done in `debug` mode. This mode *disables* inlining, and adds additional runtime checks to validate your code. These checks are very useful, and are part of what makes Rust safe. But they don't do much good when your tests *don't even run* due to a stack overflow. You can carry on testing in `release` mode, but it's far from ideal. Your best bet is to test on your native architecture.

## Some closing tips
* Try to have clean separation between JavaScript and WASM.\
This will help keep your WASM code portable, and make testing on other platforms easier
* Don't be afraid to use `unsafe{}` in Rust\
`unsafe{}` is required for some operations. Pay attention to what you're doing! The borrow checker won't save you here
