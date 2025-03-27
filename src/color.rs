#![allow(unused_unsafe)]
use std::arch::wasm32::{f32x4, f32x4_div, f32x4_extract_lane, f32x4_max, f32x4_min, f32x4_mul, f32x4_splat, f32x4_sqrt};

use crate::vector3::Vector3;

pub fn write_color(pixel_color: Vector3, reservoir: &mut Vec<f32>, pixel_index: usize) {
    // Overwrite if zero (default value, else add)
    if reservoir[pixel_index] != 0.0 {
        reservoir[pixel_index] += unsafe { f32x4_extract_lane::<0>(pixel_color.0) };
        reservoir[pixel_index + 1] += unsafe { f32x4_extract_lane::<1>(pixel_color.0) };
        reservoir[pixel_index + 2] += unsafe { f32x4_extract_lane::<2>(pixel_color.0) };
    }

    else {
        reservoir[pixel_index] = unsafe { f32x4_extract_lane::<0>(pixel_color.0) };
        reservoir[pixel_index + 1] = unsafe { f32x4_extract_lane::<1>(pixel_color.0) };
        reservoir[pixel_index + 2] = unsafe { f32x4_extract_lane::<2>(pixel_color.0) };
    }
}

pub fn gamma_correct_average(texture: &mut Vec<u8>, reservoir: &Vec<f32>, sample_count: u32) {
    // Extract pixel from reservoir
    let mut pixel_index = 0;
    while pixel_index < reservoir.len() {
        let pixel_color = unsafe { f32x4(reservoir[pixel_index], reservoir[pixel_index + 1], reservoir[pixel_index + 2], 0.0) };
        
        // First average
        let div = unsafe { f32x4_splat(sample_count as f32) };
        let avg_color = unsafe { f32x4_div(pixel_color, div) };

        // Then gamma correct
        let rgba_gamma = unsafe { f32x4_sqrt(avg_color) };
        
        let lower_bound = unsafe { f32x4_splat(0.0) };
        let upper_bound = unsafe { f32x4_splat(1.0) };
        
        let clamped = unsafe { f32x4_min(f32x4_max(rgba_gamma, lower_bound), upper_bound) };
        
        let color = unsafe { f32x4_mul(f32x4_splat(255.999), clamped) };

        texture[pixel_index] = unsafe { f32x4_extract_lane::<0>(color) } as u8;
        texture[pixel_index + 1] = unsafe { f32x4_extract_lane::<1>(color) } as u8;
        texture[pixel_index + 2] = unsafe { f32x4_extract_lane::<2>(color) } as u8;

        pixel_index += 3;
    }
}