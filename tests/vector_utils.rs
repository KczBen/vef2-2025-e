use vef2_2025_e::vector_utils::*;
use vef2_2025_e::vector3::Vector3;
use wasm_bindgen_test::*;
use approx::abs_diff_eq;

#[wasm_bindgen_test]
fn test_random_f32_range() {
    let min = 0.0;
    let max = 1.0;
    for _ in 0..1000 {
        let val = fastrand::f32();
        assert!(val >= min && val <= max);
    }
}

#[wasm_bindgen_test]
fn test_random_vec3() {
    for _ in 0..100 {
        let v = random_vec3();
        assert!(v.x() >= 0.0 && v.x() <= 1.0);
        assert!(v.y() >= 0.0 && v.y() <= 1.0);
        assert!(v.z() >= 0.0 && v.z() <= 1.0);
    }
}

#[wasm_bindgen_test]
fn test_random_vec3_range() {
    let min = 5.0;
    let max = 10.0;
    for _ in 0..100 {
        let v = random_vec3_range(min, max);
        assert!(v.x() >= min && v.x() <= max);
        assert!(v.y() >= min && v.y() <= max);
        assert!(v.z() >= min && v.z() <= max);
    }
}

#[wasm_bindgen_test]
fn test_random_vec3_unit() {
    for _ in 0..100 {
        let v = random_vec3_unit();
        let len_sq = v.norm_squared();
        assert!(abs_diff_eq!(len_sq, 1.0, epsilon = 1e-5));
    }
}

#[wasm_bindgen_test]
fn test_random_vec3_sphere() {
    for _ in 0..100 {
        let v = random_vec3_sphere();
        let len_sq = v.norm_squared();
        assert!(len_sq <= 1.0);
    }
}

#[wasm_bindgen_test]
fn test_random_vec3_hemisphere() {
    let normal = Vector3::new(0.0, 1.0, 0.0);
    for _ in 0..100 {
        let v = random_vec3_hemisphere(normal);
        assert!(Vector3::dot(v, normal) > 0.0);
    }
}

#[wasm_bindgen_test]
fn test_near_zero() {
    let zero = Vector3::new(0.0, 0.0, 0.0);
    assert!(near_zero(zero));
    
    let small = Vector3::new(1e-6, 1e-6, 1e-6);
    assert!(near_zero(small));
    
    let large = Vector3::new(1e-4, 0.0, 0.0);
    assert!(!near_zero(large));
}

#[wasm_bindgen_test]
fn test_reflect() {
    let vector = Vector3::new(1.0, 0.0, 0.0);
    let normal = Vector3::new(1.0, 0.0, 0.0);
    let reflected = reflect(vector, normal);
    console_log!("reflected.x was {}", reflected.x());
    assert!(abs_diff_eq!(reflected.x(), 1.0, epsilon = 1e-5));
    assert!(abs_diff_eq!(reflected.y(), 0.0, epsilon = 1e-5));
    assert!(abs_diff_eq!(reflected.z(), 0.0, epsilon = 1e-5));
    
    let vector = Vector3::new(0.0, 1.0, 0.0);
    let normal = Vector3::new(1.0, 0.0, 0.0);
    let reflected = reflect(vector, normal);
    assert!(abs_diff_eq!(reflected.x(), 0.0, epsilon = 1e-5));
    assert!(abs_diff_eq!(reflected.y(), -1.0, epsilon = 1e-5));
    assert!(abs_diff_eq!(reflected.z(), 0.0, epsilon = 1e-5));
}

#[wasm_bindgen_test]
fn test_refract() {
    let uv = Vector3::new(1.0, 0.0, 0.0);
    let n = Vector3::new(1.0, 0.0, 0.0);
    let etai_over_etat = 1.0;
    let refracted = refract(uv, n, etai_over_etat);
    assert!(abs_diff_eq!(refracted.x(), 1.0, epsilon = 1e-5));
    assert!(abs_diff_eq!(refracted.y(), 0.0, epsilon = 1e-5));
    assert!(abs_diff_eq!(refracted.z(), 0.0, epsilon = 1e-5));
    
    let uv = Vector3::new(0.0, 1.0, 0.0);
    let n = Vector3::new(0.0, 1.0, 0.0);
    let etai_over_etat = 1.0;
    let refracted = refract(uv, n, etai_over_etat);
    assert!(abs_diff_eq!(refracted.x(), 0.0, epsilon = 1e-5));
    assert!(abs_diff_eq!(refracted.y(), 1.0, epsilon = 1e-5));
    assert!(abs_diff_eq!(refracted.z(), 0.0, epsilon = 1e-5));
    
    // Test refraction at 45 degrees
    let uv = Vector3::new(1.0, 1.0, 0.0).normalize();
    let n = Vector3::new(0.0, 1.0, 0.0);
    let etai_over_etat = 1.0;
    let refracted = refract(uv, n, etai_over_etat);
    // Expected refraction should have same y-component
    assert!(abs_diff_eq!(refracted.y(), 1.0 / f32::sqrt(2.0), epsilon = 1e-5));
}
