// Shared memory struct between JS and WASM
// This contains all the things that may be accessed from both sides

// repr C so the compiler keeps the order of things
#[repr(C)]
#[derive(Debug, Default)]
pub struct SharedMem {
    // Camera settings
    pub target_width: u32,
    pub target_height: u32,
    pub samples_per_pixel: u32,
    pub max_bounces: u32,
    pub origin_x: f32,
    pub origin_y: f32,
    pub origin_z: f32,
    pub look_at_x: f32,
    pub look_at_y: f32,
    pub look_at_z: f32,

    // Notifier flags
    pub texture_changed: u32,
    pub settings_changed: u32,
    pub busy: u32,
}