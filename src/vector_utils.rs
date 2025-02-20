// Some extras on top of Nalgebra and fastrand

use nalgebra::{Normed, Vector, Vector3};
use fastrand::f32;

fn random_f32_range(min: f32, max: f32) -> f32 {
    return fastrand::f32() * (max - min) + min;
}

// Might be useful later
#[allow(dead_code)]
fn random_vec3() -> Vector3<f32> {
    return Vector3::new(f32(), f32(), f32());
}

#[inline(always)]
fn random_vec3_range(min: f32, max: f32) -> Vector3<f32> {
    return Vector3::new(random_f32_range(min, max), random_f32_range(min, max), random_f32_range(min, max));
}

#[inline(always)]
pub fn random_vec3_unit() -> Vector3<f32>  {
    loop {
        let p = random_vec3_range(-1.0, 1.0);
        let len_sqr = p.norm_squared();

        if 1e-160 < len_sqr && len_sqr <= 1.0 {
            return p / len_sqr.sqrt();
        }
    }
}

#[inline(always)]
pub fn random_vec3_sphere() -> Vector3<f32> {
    loop {
        let p = random_vec3_range(-1.0, 1.0);
        if p.norm_squared() < 1.0 {
            return p;
        }
    }
}

#[allow(dead_code)]
#[inline(always)]
pub fn random_vec3_hemisphere(normal: &Vector3<f32>) -> Vector3<f32> {
    let vec3_on_unit_sphere = random_vec3_unit();

    if Vector3::dot(&vec3_on_unit_sphere, normal) > 0.0 {
        return vec3_on_unit_sphere;
    }

    else {
        return -1.0 * vec3_on_unit_sphere;
    }
}

#[inline(always)]
pub fn near_zero(vector: Vector3<f32>) -> bool {
    let s = 1e-8;
    return vector.x < s && vector.y < s && vector.z < s;
}

#[inline(always)]
pub fn reflect(vector: &Vector3<f32 >, normal: &Vector3<f32>) -> Vector3<f32> {
    return vector - 2.0 * Vector3::dot(&vector, &normal) * normal;
}

#[inline(always)]
pub fn refract(uv: &Vector3<f32>, n: &Vector3<f32>, etai_over_etat: f32) -> Vector3<f32> {
    let cost_theta = f32::min(-uv.dot(n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cost_theta * n);
    let r_out_parallel = -f32::sqrt(f32::abs(1.0 - r_out_perp.norm_squared())) * n;
    return r_out_perp + r_out_parallel;
}