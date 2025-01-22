pub mod sphere {
    use crate::interval::Interval;
    use crate::scene_object::scene_object::HitRecord;
    use crate::ray::ray;
    use nalgebra::Vector3;
    use crate::scene_object::scene_object::SceneObject;

    pub struct Sphere {
        centre: Vector3<f64>,
        radius: f64,
    }

    impl Sphere {
        pub fn new(centre: Vector3<f64>, radius: f64) -> Self {
            Sphere {
                centre,
                radius: radius.max(0.0),
            }
        }
    }

    impl SceneObject for Sphere {
        fn hit(&self, ray: &ray::Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
            let oc = self.centre - ray.origin();
            let a = ray.direction().norm().powi(2);
            let h = nalgebra::Vector::dot(&ray.direction(), &oc);
            let c = oc.norm().powi(2) - self.radius * self.radius;

            let discriminant = h * h - a * c;

            if discriminant < 0.0 {
                return false;
            }

            let discriminant_sqrt = discriminant.sqrt();

            let mut root = (h - discriminant_sqrt) / a;
            if !ray_t.surrounds(root) {
                root = (h + discriminant_sqrt) / a;
                if !ray_t.surrounds(root) {
                    return false;
                }
            }

            record.t = root;
            record.point = ray.at(record.t);
            let outward_normal = (record.point - self.centre) / self.radius;
            record.set_face_normal(ray, &outward_normal);

            return true;
        }
    }
}