pub mod scene_object {
    use std::sync::Arc;

    use nalgebra::Vector3;

    use crate::{interval::Interval, material::Material, ray::ray};

    #[derive(Clone)]
    pub struct HitRecord {
        pub point: Vector3<f32>,
        pub normal: Vector3<f32>,
        pub material: Arc<dyn Material>,
        pub t: f32,
        pub front_face: bool,
    }

    impl HitRecord {
        /*pub fn set_face_normal(&mut self, ray: &ray::Ray, out_normal: &Vector3<f32>) {
            self.front_face = nalgebra::Vector::dot(&ray.direction(), &out_normal) < 0.0;
            
            if self.front_face {
                self.normal = *out_normal
            }

            else {
                self.normal = -*out_normal
            }
        }*/

        pub fn new(material: Arc<dyn Material>) -> Self {
            Self {
                point: Vector3::new(0.0, 0.0, 0.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
                material,
                t: 0.0,
                front_face: false
            }
        }
    }

    pub trait SceneObject: Sync + Send {
        fn hit(&self, ray: &ray::Ray, ray_t: Interval) -> Option<HitRecord>;
    }
}