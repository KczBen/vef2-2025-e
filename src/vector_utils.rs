// Some extras on top of Nalgebra and fastrand

use nalgebra::Vector3;
use fastrand::f64;

fn random_f64_range(min: f64, max: f64) -> f64 {
    return fastrand::f64() * (max - min) + min;
}

// Might be useful later
#[allow(dead_code)]
fn random_vec3() -> Vector3<f64> {
    return Vector3::new(f64(), f64(), f64());
}

fn random_vec3_range(min: f64, max: f64) -> Vector3<f64> {
    return Vector3::new(random_f64_range(min, max), random_f64_range(min, max), random_f64_range(min, max));
}

pub fn random_vec3_unit() -> Vector3<f64>  {
    loop {
        let p = random_vec3_range(-1.0, 1.0);
        let len_sqr = p.norm_squared();

        if 1e-160 < len_sqr && len_sqr <= 1.0 {
            return p / len_sqr.sqrt();
        }
    }
}

pub fn random_vec3_hemisphere(normal: &Vector3<f64>) -> Vector3<f64> {
    let vec3_on_unit_sphere = random_vec3_unit();

    if Vector3::dot(&vec3_on_unit_sphere, normal) > 0.0 {
        return vec3_on_unit_sphere;
    }

    else {
        return -1.0 * vec3_on_unit_sphere;
    }
}