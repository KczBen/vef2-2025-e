#![allow(unused_unsafe)]
use std::arch::wasm32::{f32x4_extract_lane, f32x4_max, f32x4_min, f32x4_mul, f32x4_splat, f32x4_sqrt};

use crate::vector3::Vector3;

pub fn write_color(pixel_color: Vector3, texture: &mut Vec<u8>, pixel_index: usize) {
    let rgba_gamma = unsafe { f32x4_sqrt(pixel_color.0) };

    let lower_bound = unsafe { f32x4_splat(0.0) };
    let upper_bound = unsafe { f32x4_splat(1.0) };

    let clamped = unsafe { f32x4_min(f32x4_max(rgba_gamma, lower_bound), upper_bound) };

    let color = unsafe { f32x4_mul(f32x4_splat(255.999), clamped) };

    texture[pixel_index] = unsafe { f32x4_extract_lane::<0>(color) } as u8;
    texture[pixel_index + 1] = unsafe { f32x4_extract_lane::<1>(color) } as u8;
    texture[pixel_index + 2] = unsafe { f32x4_extract_lane::<2>(color) } as u8;
}