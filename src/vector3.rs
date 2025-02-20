use core::arch::wasm32;
use std::arch::wasm32::{f32x4, f32x4_mul, f32x4_splat};

pub struct Vector3<T> {
    x: T,
    y: T,
    z: T
}

impl std::ops::Mul<f32> for Vector3<f32> {
    type Output = Self;
    // Scalar element-wise multiplication
    fn mul(self, rhs: f32) -> Self::Output {
        let packed = f32x4(self.x, self.y, self.y, 1.0);
        return unsafe { f32x4_mul(f32x4_splat(rhs), packed) };
    }
}

impl std::ops::Mul<Vector3<f32>> for Vector3<f32> {
    type Output = Self;
    // Vector dot product
    fn mul(self, rhs: Vector3<f32>) -> Self::Output {
        let packed_self = f32x4(self.x, self.y, self.z, 1.0);
        let packed_rhs = f32x4(rhs.x, rhs.y, rhs.z, 1.0);

        // Lane-wise multiplication, then sum the lanes somehow
        return unsafe { f32x4_mul(packed_self, packed_rhs) };
    }
}