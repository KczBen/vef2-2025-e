// xorshift code straight from Wikipedia, but extra rusty

use crate::RNG;

pub struct Xorshift32State {
    a: u32,
}

impl Xorshift32State {
    // Creates a new Xorshift32State struct
    // Seed may not be zero
    pub fn new(seed: u32) -> Self {
        return Self { a: seed };
    }

    pub fn next(&mut self) -> f32 {
        let mut x = self.a;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
    
        self.a = x;
    
        return x as f32 / u32::MAX as f32;
    }
}


pub fn random_f32() -> f32 {
    RNG.with(|rng| rng.borrow_mut().next())
}