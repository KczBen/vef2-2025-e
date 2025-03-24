pub mod object_list {
    use std::sync::Arc;

    use crate::{interval::Interval, ray::ray, scene_object::scene_object::{HitRecord, SceneObject}};

    #[derive(Default)]
    pub struct ObjectList {
        pub objects: Vec<Arc<dyn SceneObject>>,
    }

    impl ObjectList {
        pub fn add(&mut self, object: Arc<dyn SceneObject>) {
            self.objects.push(object);
        }
    }

    impl SceneObject for ObjectList {
        fn hit(&self, ray: &ray::Ray, ray_t: Interval) -> Option<HitRecord> {
            //let mut temp_rec: HitRecord = HitRecord::new(record.material.clone());
            let mut hit_anything  = None;
            let mut closest_hit = ray_t.max;

            for object in &self.objects {
                if let Some(hit) = object.hit(ray, Interval::new(ray_t.min, closest_hit)) {
                    hit_anything = Some(hit.clone());
                    closest_hit = hit.t;
                    //*record = temp_rec.clone();
                }
            }
            return hit_anything;
        }
    }
}