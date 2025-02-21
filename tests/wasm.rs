use wasm_bindgen_test::*;
use vef2_2025_e::vector3::Vector3;
use std::f32::consts::SQRT_2;
use std::f32::EPSILON;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_new() {
    let v = Vector3::new(1.0, 2.0, 3.0);
    assert_eq!(v.x(), 1.0);
    assert_eq!(v.y(), 2.0);
    assert_eq!(v.z(), 3.0);
}

#[wasm_bindgen_test]
fn test_default() {
    let v = Vector3::default();
    assert_eq!(v.x(), 0.0);
    assert_eq!(v.y(), 0.0);
    assert_eq!(v.z(), 0.0);
}

#[wasm_bindgen_test]
fn test_norm() {
    let v = Vector3::new(3.0, 4.0, 0.0);
    let norm = v.norm();
    assert!((norm - 5.0).abs() < EPSILON);
}

#[wasm_bindgen_test]
fn test_norm_squared() {
    let v = Vector3::new(3.0, 4.0, 0.0);
    let norm_sq = v.norm_squared();
    assert!((norm_sq - 25.0).abs() < EPSILON);
}

#[wasm_bindgen_test]
fn test_normalize() {
    let v = Vector3::new(3.0, 4.0, 0.0);
    let unit = v.normalize();
    assert!((unit.x() - 0.6).abs() < EPSILON);
    assert!((unit.y() - 0.8).abs() < EPSILON);
    assert!((unit.z() - 0.0).abs() < EPSILON);
}

#[wasm_bindgen_test]
#[should_panic]
fn test_normalize_zero() {
    let v = Vector3::default();
    v.normalize();
}

#[wasm_bindgen_test]
fn test_cross() {
    let i = Vector3::new(1.0, 0.0, 0.0);
    let j = Vector3::new(0.0, 1.0, 0.0);
    let k = Vector3::new(0.0, 0.0, 1.0);
    
    assert_eq!(i.cross(j), k);
    assert_eq!(j.cross(i), Vector3::new(0.0, 0.0, -1.0));
    assert_eq!(i.cross(i), Vector3::default());
}

#[wasm_bindgen_test]
fn test_dot() {
    let v = Vector3::new(1.0, 2.0, 3.0);
    let u = Vector3::new(4.0, 5.0, 6.0);
    assert_eq!(v.dot(u), 1.0*4.0 + 2.0*5.0 + 3.0*6.0);
}

#[wasm_bindgen_test]
fn test_component_mul() {
    let v = Vector3::new(2.0, 3.0, 4.0);
    let u = Vector3::new(0.5, 1.0, 2.0);
    let result = v.component_mul(u);
    assert_eq!(result.x(), 1.0);
    assert_eq!(result.y(), 3.0);
    assert_eq!(result.z(), 8.0);
}

#[wasm_bindgen_test]
fn test_scalar_mul() {
    let v = Vector3::new(1.0, 2.0, 3.0);
    let result = v * 2.0;
    assert_eq!(result.x(), 2.0);
    assert_eq!(result.y(), 4.0);
    assert_eq!(result.z(), 6.0);
}

#[wasm_bindgen_test]
fn test_scalar_div() {
    let v = Vector3::new(2.0, 4.0, 6.0);
    let result = v / 2.0;
    assert_eq!(result.x(), 1.0);
    assert_eq!(result.y(), 2.0);
    assert_eq!(result.z(), 3.0);
}

#[wasm_bindgen_test]
fn test_add() {
    let v = Vector3::new(1.0, 2.0, 3.0);
    let u = Vector3::new(4.0, 5.0, 6.0);
    let result = v + u;
    assert_eq!(result.x(), 5.0);
    assert_eq!(result.y(), 7.0);
    assert_eq!(result.z(), 9.0);
}

#[wasm_bindgen_test]
fn test_sub() {
    let v = Vector3::new(5.0, 7.0, 9.0);
    let u = Vector3::new(1.0, 2.0, 3.0);
    let result = v - u;
    assert_eq!(result.x(), 4.0);
    assert_eq!(result.y(), 5.0);
    assert_eq!(result.z(), 6.0);
}

#[wasm_bindgen_test]
fn test_add_assign() {
    let mut v = Vector3::new(1.0, 2.0, 3.0);
    let u = Vector3::new(4.0, 5.0, 6.0);
    v += u;
    assert_eq!(v.x(), 5.0);
    assert_eq!(v.y(), 7.0);
    assert_eq!(v.z(), 9.0);
}
