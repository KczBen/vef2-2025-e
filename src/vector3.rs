use std::arch::wasm32::{f32x4, f32x4_add, f32x4_div, f32x4_extract_lane, f32x4_mul, f32x4_splat, f32x4_sqrt, f32x4_sub, i32x4_shuffle, v128};

#[derive(Clone, Copy)]
pub struct Vector3(v128);

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        return Self(f32x4(x, y, z, 0.0));
    }

    #[inline]
    pub fn x(self) -> f32 { unsafe { f32x4_extract_lane::<0>(self.0) } }
    #[inline]
    pub fn y(self) -> f32 { unsafe { f32x4_extract_lane::<1>(self.0) } }
    #[inline]
    pub fn z(self) -> f32 { unsafe { f32x4_extract_lane::<2>(self.0) } }

    pub fn norm(self) -> f32 {
        // x^2, y^2, z^2, w^2 (w is 0)
        let squared = unsafe { f32x4_mul(self.0, self.0) };

        // move them into different places
        // New order: z^2, w^2, x^2, y^2
        let shuf1 = unsafe { i32x4_shuffle::<2, 3, 0, 1>(squared, squared) };

        // add the shuffled and original vectors
        // (x^2 + z^2), (y^2 + w^2), (z^2 + x^2), (w^2 + y^2)
        let sum1 = unsafe { f32x4_add(squared, shuf1) };

        // reorder again
        // New order: (y^2 + w^2), (x^2 + z^2), (w^2 + y^2), (z^2 + x^2)
        let shuf2 = unsafe { i32x4_shuffle::<1, 0, 3, 2>(sum1, sum1) };

        // add the results
        // each lane now holds (x^2 + y^2 + z^2 + w^2)
        let sum_all = unsafe { f32x4_add(sum1, shuf2) };

        // take the square root of each lane
        let norm_vec = unsafe { f32x4_sqrt(sum_all) };

        // extract one lane (they’re all equal)
        return unsafe { f32x4_extract_lane::<0>(norm_vec) }
    }

    pub fn norm_squared(self) -> f32 {
        // x^2, y^2, z^2, w^2 (w is 0)
        let squared = unsafe { f32x4_mul(self.0, self.0) };

        // move them into different places
        // New order: z^2, w^2, x^2, y^2
        let shuf1 = unsafe { i32x4_shuffle::<2, 3, 0, 1>(squared, squared) };

        // add the shuffled and original vectors
        // (x^2 + z^2), (y^2 + w^2), (z^2 + x^2), (w^2 + y^2)
        let sum1 = unsafe { f32x4_add(squared, shuf1) };

        // reorder again
        // New order: (y^2 + w^2), (x^2 + z^2), (w^2 + y^2), (z^2 + x^2)
        let shuf2 = unsafe { i32x4_shuffle::<1, 0, 3, 2>(sum1, sum1) };

        // add the results
        // each lane now holds (x^2 + y^2 + z^2 + w^2)
        let sum_all = unsafe { f32x4_add(sum1, shuf2) };

        // extract one lane (they’re all equal)
        return unsafe { f32x4_extract_lane::<0>(sum_all) }
    }

    pub fn normalize(self) -> Vector3 {
        let norm = self.norm();
        let inv_norm = 1.0 / norm;
        let inv = unsafe { f32x4_splat(inv_norm) };
        let normalized = unsafe { f32x4_mul(self.0, inv) };
        
        return Self(normalized);
    }

    pub fn cross(self, rhs: Vector3) -> Vector3 {
        // Left hand side of subtraction
        // l.y * r.z, l.z * r.x, l.x * r.y

        // Re-arrange left side for multiplication
        // y, z, x, w
        let left_shuffle = unsafe { i32x4_shuffle::<1, 3, 0, 4>(self.0, self.0) };
        // Right side
        // z, x, y, w
        let right_shuffle = unsafe { i32x4_shuffle::<3, 0, 1, 4>(rhs.0, rhs.0) };

        let mul_left = unsafe { f32x4_mul(left_shuffle, right_shuffle) };

        // Right hand side of subtraction
        // l.z * r.y, l.x * r.z, l.y * r.x

        let left_shuffle = unsafe { i32x4_shuffle::<2, 0, 1, 4>(self.0, self.0) };
        let right_shuffle = unsafe { i32x4_shuffle::<1, 2, 0, 4>(rhs.0, rhs.0) };

        let mul_right = unsafe { f32x4_mul(left_shuffle, right_shuffle) };

        return Self(unsafe { f32x4_sub(mul_left, mul_right) });
    }

    pub fn dot(self, rhs: Vector3) -> f32 {
        let lane_product = unsafe { f32x4_mul(self.0, rhs.0) };
        // There's no way to sum the lanes (yet?)
        let x = unsafe { f32x4_extract_lane::<0>(lane_product) };
        let y = unsafe { f32x4_extract_lane::<1>(lane_product) };
        let z = unsafe { f32x4_extract_lane::<2>(lane_product) };

        return x + y + z;
    }

    pub fn component_mul(self, rhs: Vector3) -> Vector3 {
        return Self(unsafe { f32x4_mul(self.0, rhs.0) });
    }
}

impl std::ops::Mul<f32> for Vector3 {
    type Output = Self;
    // Scalar element-wise multiplication
    fn mul(self, rhs: f32) -> Self::Output {
        return Self(unsafe { f32x4_mul( self.0, f32x4_splat(rhs)) });
    }
}

impl std::ops::Mul<Vector3> for f32 {
    type Output = Vector3;
    // Scalar element-wise multiplication
    fn mul(self, rhs: Vector3) -> Vector3 {
        return Vector3(unsafe { f32x4_mul(f32x4_splat(self), rhs.0) });
    }
}

impl std::ops::Div<f32> for Vector3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        return Self(unsafe { f32x4_div(self.0, f32x4_splat(rhs)) });
    }
}

impl std::ops::Div<Vector3> for Vector3 {
    type Output = Self;

    fn div(self, rhs: Vector3) -> Self::Output {
        return Self(unsafe { f32x4_div(self.0, rhs.0) });
    }
}

impl std::ops::Add<f32> for Vector3 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        return Self(unsafe { f32x4_add(self.0, f32x4_splat(rhs)) });
    }
}

impl std::ops::Add<Vector3> for Vector3 {
    type Output = Self;

    fn add(self, rhs: Vector3) -> Self::Output {
        return Self(unsafe { f32x4_add(self.0, rhs.0) });
    }
}

impl std::ops::AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        *self = Self(unsafe { f32x4_add(self.0, rhs.0) });
    }
}

impl std::ops::Sub<Vector3> for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Vector3) -> Self::Output {
        return Self(unsafe { f32x4_sub(self.0, rhs.0) });
    }
}

impl Default for Vector3 {
    fn default() -> Self {
        return Self(unsafe { f32x4_splat(0.0) });
    }
}