use nalgebra::Vector3;

use crate::console_log;
use crate::log;

pub fn write_color(pixel_color: &Vector3<f64>, texture: &mut Vec<u8>, pixel_index: usize) {
    let r = pixel_color[0];
    let g = pixel_color[1];
    let b = pixel_color[2];

    let rbyte = (255.999 * r) as u32;
    let gbyte = (255.999 * g) as u32;
    let bbyte = (255.999 * b) as u32;

    console_log!("{} {} {}", rbyte, gbyte, bbyte);
    texture[pixel_index] = rbyte as u8;
    texture[pixel_index + 1] = gbyte as u8;
    texture[pixel_index + 2] = bbyte as u8;
}