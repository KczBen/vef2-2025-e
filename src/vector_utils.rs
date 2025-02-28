// Some extras on top of Nalgebra and fastrand

use crate::vector3::Vector3;
use crate::rng;

pub fn random_f32_range(min: f32, max: f32) -> f32 {
    return rng::random_f32() * (max - min) + min;
}

// Might be useful later
#[allow(dead_code)]
pub fn random_vec3() -> Vector3 {
    return Vector3::new(rng::random_f32(), rng::random_f32(), rng::random_f32());
}

#[inline(always)]
pub fn random_vec3_range(min: f32, max: f32) -> Vector3 {
    return Vector3::new(random_f32_range(min, max), random_f32_range(min, max), random_f32_range(min, max));
}

#[inline(always)]
pub fn random_vec3_unit() -> Vector3  {
    loop {
        let p = random_vec3_range(-1.0, 1.0);
        let len_sqr = p.norm_squared();

        if 0.0 < len_sqr && len_sqr <= 1.0 {
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