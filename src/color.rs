use nalgebra::Vector3;

use crate::interval::Interval;

#[inline(always)]
fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }

    return 0.0;
}

pub fn write_color(pixel_color: &Vector3<f64>, texture: &mut Vec<u8>, pixel_index: usize) {
    let r = linear_to_gamma(pixel_color[0]);
    let g = linear_to_gamma(pixel_color[1]);
    let b = linear_to_gamma(pixel_color[2]);

    let intensity = Interval::new(0.0, 0.999);

    let rbyte = (255.999 * intensity.clamp(r)) as u32;
    let gbyte = (255.999 * intensity.clamp(g)) as u32;
    let bbyte = (255.999 * intensity.clamp(b)) as u32;

    texture[pixel_index] = rbyte as u8;
    texture[pixel_index + 1] = gbyte as u8;
    texture[pixel_index + 2] = bbyte as u8;
}