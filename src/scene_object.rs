pub mod scene_object {
    use nalgebra::Vector3;

    use crate::{interval::Interval, ray::ray};

    #[derive(Default, Clone, Copy)]
    pub struct HitRecord {
        pub point: Vector3<f64>,
        pub normal: Vector3<f64>,
        pub t: f64,
        pub front_face: bool,
    }

    impl HitRecord {
        pub fn set_face_normal(&mut self, ray: &ray::Ray, out_normal: &Vector3<f64>) {
            self.front_face = nalgebra::Vector::dot(&ray.direction(), &out_normal) < 0.0;
            
            if self.front_face {
                self.normal = *out_normal
            }

            else {
                self.normal = -*out_normal
            }
        }
    }

    pub trait SceneObject {
        fn hit(&self, ray: &ray::Ray, ray_t: Interval, record: &mut HitRecord) -> bool;
    }
}