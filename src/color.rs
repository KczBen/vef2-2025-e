#![allow(unused_unsafe)]
use std::arch::wasm32::{f32x4_extract_lane, f32x4_max, f32x4_min, f32x4_mul, f32x4_nearest, f32x4_splat, f32x4_sqrt, u8x16_extract_lane};

use crate::vector3::Vector3;

use crate::interval::Interval;

use crate::{log, console_log};

#[inline(always)]
fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }

    return 0.0;
}

#[inline(never)]
pub fn write_color(pixel_color: Vector3, texture: &mut Vec<u8>, pixel_index: usize) {
    /*let r = linear_to_gamma(pixel_color.x());
    let g = linear_to_gamma(pixel_color.y());
    let b = linear_to_gamma(pixel_color.z());

    let intensity = Interval::new(0.0, 0.999);

    let rbyte = (255.999 * intensity.clamp(r)) as u32;
    let gbyte = (255.999 * intensity.clamp(g)) as u32;
    let bbyte = (255.999 * intensity.clamp(b)) as u32;

    texture[pixel_index] = rbyte as u8;
    texture[pixel_index + 1] = gbyte as u8;
    texture[pixel_index + 2] = bbyte as u8;*/

    let rgba_gamma = unsafe { f32x4_sqrt(pixel_color.0) };

    let lower_bound = unsafe { f32x4_splat(0.0) };
    let upper_bound = unsafe { f32x4_splat(1.0) };

    let clamped = unsafe { f32x4_min(f32x4_max(rgba_gamma, lower_bound), upper_bound) };

    // Experimental: Pack more colours in one vector
    // u8x16 should fit 4 pixels as RGBA, and only use SIMD operations
    // let color = unsafe { f32x4_nearest(f32x4_mul(f32x4_splat(255.999), clamped)) };
    let color = unsafe { f32x4_mul(f32x4_splat(255.999), clamped) };
    
    // console_log!("u8 lane 0 is {}", unsafe { u8x16_extract_lane::<0>(color) });


    texture[pixel_index] = unsafe { f32x4_extract_lane::<0>(color) } as u8;
    texture[pixel_index + 1] = unsafe { f32x4_extract_lane::<1>(color) } as u8;
    texture[pixel_index + 2] = unsafe { f32x4_extract_lane::<2>(color) } as u8;
}