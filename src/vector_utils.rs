// Compiler is still wrong
#![allow(unused_unsafe)]
use std::arch::wasm32::{f32x4_add, f32x4_mul, f32x4_splat, f32x4_sub};

use crate::vector3::Vector3;
use crate::rng;

#[inline(always)]
pub fn random_vec3_range(min: f32, max: f32) -> Vector3 {
    let mut vec = rng::random_v128();
    let min = unsafe { f32x4_splat(min) };
    let max = unsafe { f32x4_splat(max) };
    let range = unsafe { f32x4_sub(max, min) };

    vec = unsafe { f32x4_mul(vec, range) };
    vec = unsafe { f32x4_add(vec, min) };
    return Vector3(vec);
}

#[inline(always)]
pub fn random_vec3_unit() -> Vector3  {
    loop {
        let p = random_vec3_range(-1.0, 1.0);
        let len_sqr = p.norm_squared();

        if 1e-160 < len_sqr && len_sqr <= 1.0 {
            return p.normalize();
        }
    }
}

#[inline(always)]
pub fn random_vec3_sphere() -> Vector3 {
    loop {
        let p = random_vec3_range(-1.0, 1.0);
        if p.norm_squared() < 1.0 {
            return p;
        }
    }
}

#[allow(dead_code)]
#[inline(always)]
pub fn random_vec3_hemisphere(normal: Vector3) -> Vector3 {
    let vec3_on_unit_sphere = random_vec3_unit();

    if Vector3::dot(vec3_on_unit_sphere, normal) > 0.0 {
        return vec3_on_unit_sphere;
    }

    else {
        return -1.0 * vec3_on_unit_sphere;
    }
}

#[inline(always)]
pub fn near_zero(vector: Vector3) -> bool {
    let s = 1e-5;
    return vector.x() < s && vector.y() < s && vector.z() < s;
}

#[inline(always)]
pub fn reflect(vector: Vector3, normal: Vector3) -> Vector3 {
    return vector - 2.0 * Vector3::dot(vector, normal) * normal;
}

#[inline(always)]
pub fn refract(uv: Vector3, n: Vector3, etai_over_etat: f32) -> Vector3 {
    let cost_theta = f32::min(-uv.dot(n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cost_theta * n);
    let r_out_parallel = -f32::sqrt(f32::abs(1.0 - r_out_perp.norm_squared())) * n;
    return r_out_perp + r_out_parallel;
}