# This contains the presentation topics and some notes

## What is WASM
Web Assembly is sort of the second language of the web, next to JavaScript. It's a statically typed language and is supported by all modern browsers. It runs and compiles faster than JavaScript. There's one slight issue with it - it looks like *this*:

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

The types are easy enough:
i: integer
32: width

So an i32 is a 32 bit integer. The specification defines it as neither signer nor unsigned. But the rest is a mess. As the name suggest, this is an assembly-like language. While you can write it by hand, the intended way to use it is to write code in a higher level langauge, and then compile it into WASM.

### Language considerations
There are a number of languages you may choose, but in practice, you will most likely end up using C, C++ or Rust. If you choose to write your code in C, keep security in mind. All of the C vulnerabilities you know carry over to WASM! While JavaScript is normally not vulnerable to buffer overflows, including Web Assembly introduces the threat of buffer overflows and possibly remote code execution.

I chose to write my project in Rust. Security isn't a concern for a path tracer, I simply like the language. It also offers a mature Web Assembly ecosystem for use in your project.

### Rust demo, print hello world to console
> This part will show how to use wasm-bindgen and wasm-pack, show the basics of Rust syntax and also introduce the limitations of the API... or lack thereof

Let's do a "Hello world" in Rust, and see how to run it in a browser

First, "Hello world" in Rust is very simple:

```
//lib.rs
fn main() {
  println!("Hello, world!");
}
```

Rust syntax follows this general form:

>Variables

Define immutable (const) variable:
let name: type = ...

Define mutable variable:
let mut name: type = ...

The Rust compiler does type inference, so defining the types is optional in most cases.

>Function declerations

```
fn name(arg1: type) -> return_type {
  ...
}
```

Note that in function signatures, defining the args type and return type is necessary. 

To get it running in a browser, we'll use two extra dependencies: `wasm-pack` and `wasm-bindgen`.

`wasm-pack` packages your `wasm` code and glue logic into an ES module for use in your main JavaScript code.

`wasm-bindgen` is used for importing JavaScript functions and exporting Rust functions.



## Limitations
Web Assembly has only one possible API - JavaScript. WASM by itself cannot take input nor give output, all communication must be done via JavaScript. Luckily, you have two options:

* Import/Export functions

WASM allows you to directly call JavaScript functions and use their return value, and you may also call WASM functions from JavaScript. This is the safe way to do things, albeit somewhat slow. On one side, you have a language with a very strong type system, and on the other you have one that is completely detached from reality. Type checking and conversion takes some time on this interface.

* Shared memory

WASM operates in a linear memory model, that is, all of the memory used by the module is in a continuous array. This can be accessed in JavaScript as an ArrayBuffer. For high performance code, you may consider reading and writing values directly to this buffer. Here be dragons! You are completely on your own with regards to type checking and memory safety.

## So still why WASM?
Despite the limitations, there's a few reasons why you might want to opt for web assembly:
* Performance - Web Assembly code is generally 10-30% faster than JavaScript.
* You might just *really* hate JavaScript - Understandable
* Port programs into web apps - Web Assembly allows you to compile the core of your desktop application into web code, only requiring you to port the user interface.
* Exclusive features - More on this later

## Graphics
> This part will talk a bit about path tracing, but not in too much detail. Should set up the next segment nicely

## Performance
We get some performance benefits just by using WebAssembly, but that’s not always guaranteed. Instead, a big boost comes from using a special type: v128. The v stands for "vector," and this type is exclusive to WebAssembly. In practice, v128 isn’t just a single type—it represents multiple values packed into one. This enables a programming paradigm called `Single Instruction Multiple Data` (`SIMD`). In standard multi-threading, work is split across multiple CPU threads, with each thread running independently and executing different operations. `SIMD` takes a different approach: it applies the same operation (`Single Instruction`) to multiple values (`Multiple Data`) at the same time - within a single CPU core.

Example: Vector addition

Normally when adding two 3D vectors, you perform:

x1 + x2
y1 + y2
z1 + z2

Each addition happens sequentially, one after another in three separate operations.

With SIMD, we define both vectors as `v128`. We can treat these vectors as 4 32-bit floats using the `f32x4` type. Then we use the `f32x4_add` operation in Rust to add the two vectors. This performs the single instruction - add - on multiple data - the 8 32-bit numbers. All of this happens in one CPU core simply by taking advantage of modern CPU instructions which are not accessible to JavaScript.

Rewriting the vector3 library to use `SIMD` results in a 25% time reduction by itself. While linear algebra is a natural fit for SIMD, it's not always this easy. Making proper use of SIMD operations requires a very different way of thinking about problems, and trying to force it into every part of your code will most likely make it an unmaintainable nightmare.

## Quirks and issues
The process isn't flawless, and there are a few points that stand out:

* Debugging is hard. Really hard.

The console in your browser doesn't tell you *Anything*. Get used to seeing the ominuous error message `Unreachable executed`. Something went wrong... but what? Even if you can figure it out with breakpoints, your function names are stripped from the WASM module. You can try using source mappings, but the compiler doesn't always output it even when you tell it to. Good luck!

* Testing is fragile.

Debugging is hard, but surely you can just test and not have bugs, right? Hah... no. It runs into one big problem: The Stack. Web Assembly has a very limited call stack size, and you will often get the error `Too much recursion` even if you don't use any. This wouldn't be an issue, if not for Rust's design. The Rust compiler very aggressively inlines functions in `release` mode. However, testing is done in `debug` mode. This mode *disables* inlining, and adds additional runtime checks to validate your code. These checks are very useful, and are part of what makes Rust safe. But they don't do much good when your tests *don't even run* due to a stack overflow. You can carry on testing in `release` mode, but it's far from ideal.