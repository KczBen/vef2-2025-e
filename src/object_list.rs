pub mod object_list {
    use crate::{interval::Interval, ray::ray, scene_object::{self, scene_object::{HitRecord, SceneObject}}};

    #[derive(Default)]
    pub struct ObjectList<'a>{
        pub objects: Vec<&'a dyn SceneObject>,
    }

    impl<'a> ObjectList<'a> {
        pub fn add(&mut self, object: &'a dyn scene_object::scene_object::SceneObject) {
            self.objects.push(object);
        }
    }

    impl<'a> SceneObject for ObjectList<'a> {
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