// Shared memory struct between JS and WASM
// This contains all the things that may be accessed from both sides

// repr C so the compiler keeps the order of things
#[repr(C)]
#[derive(Default)]
pub struct SharedMem {
    pub target_width: u32,
    pub target_height: u32,
    pub samples_per_pixel: u32,
    pub max_bounces: u32,
}