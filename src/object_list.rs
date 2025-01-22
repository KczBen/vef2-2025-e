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
        fn hit(&self, ray: &ray::Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
            let mut temp_rec: HitRecord = HitRecord::default();
            let mut hit_anything  = false;
            let mut closest_hit = ray_t.max;

            for object in &self.objects {
                if object.hit(ray, Interval::new(ray_t.min, closest_hit), &mut temp_rec) {
                    hit_anything = true;
                    closest_hit = temp_rec.t;
                    *record = temp_rec;
                }
            }
            return hit_anything;
        }
    }
}