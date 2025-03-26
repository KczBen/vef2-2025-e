// xorshift code straight from Wikipedia, but extra rusty
// It is in fact not unused, WASM SIMD requires unsafe. Compiler is worong here. Again.
#![allow(unused_unsafe)]
use std::{arch::wasm32::{f32x4_convert_u32x4, f32x4_div, f32x4_splat, u32x4, u32x4_shl, u32x4_shr, v128, v128_xor}, u32};

use crate::RNG;

pub struct Xorshift32State {
    a: u32,
    vec: v128,
}

impl Xorshift32State {
    // Creates a new Xorshift32State struct
    // Seed may not be zero
    pub fn new(seed: u32) -> Self {
        return Self { a: seed, vec: u32x4(seed, seed + 1, seed + 2, seed + 3) };
    }

    pub fn next_scalar(&mut self) -> f32 {
        let mut x = self.a;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
    
        self.a = x;
    
        return x as f32 / u32::MAX as f32;
    }

    pub fn next_vec(&mut self) -> v128 {
        let mut x = self.vec;

        x = unsafe { v128_xor(x,u32x4_shl(x, 13)) };
        x = unsafe { v128_xor(x,u32x4_shr(x, 17)) };
        x = unsafe { v128_xor(x,u32x4_shl(x, 5)) };

        self.vec = x;

        let temp = unsafe { f32x4_convert_u32x4(x) };
        let div = unsafe { f32x4_splat(u32::MAX as f32) };

        return unsafe { f32x4_div(temp, div) };
    }
}


pub fn random_f32() -> f32 {
    RNG.with(|rng| rng.borrow_mut().next_scalar())
}

pub fn random_v128() -> v128 {
    RNG.with(|rng| rng.borrow_mut().next_vec())
}