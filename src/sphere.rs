pub mod sphere {
    use std::sync::Arc;

    use crate::interval::Interval;
    use crate::material::Material;
    use crate::scene_object::scene_object::HitRecord;
    use crate::ray::ray;
    use crate::scene_object::scene_object::SceneObject;
    use crate::vector3::Vector3;

    pub struct Sphere {
        centre: Vector3,
        radius: f32,
        material: Arc<dyn Material>
    }

    impl Sphere {
        pub fn new(centre: Vector3, radius: f32, material: Arc<dyn Material>) -> Self {
            Sphere {
                centre,
                radius: radius.max(0.0),
                material,
            }
        }
    }

    impl SceneObject for Sphere {
        fn hit(&self, ray: &ray::Ray, ray_t: Interval) -> Option<HitRecord> {
            let oc = self.centre - ray.origin();
            let a = ray.direction().norm_squared();
            let h = Vector3::dot(ray.direction(), oc);
            let c = oc.norm_squared() - self.radius * self.radius;

            let discriminant = h * h - a * c;

            if discriminant < 0.0 {
                return None;
            }

            let discriminant_sqrt = discriminant.sqrt();

            let mut root = (h - discriminant_sqrt) / a;
            if !ray_t.surrounds(root) {
                root = (h + discriminant_sqrt) / a;
                if !ray_t.surrounds(root) {
                    return None;
                }
            }

            let t = root;
            let point = ray.at(t);
            let outward_normal = (point - self.centre) / self.radius;
            let front_face = ray.direction().dot(outward_normal) < 0.0;
            let normal = if front_face { outward_normal } else { -1.0 * outward_normal };
            let material = self.material.clone();

            return Some(HitRecord { point, normal, material, t, front_face });
        }
    }
}