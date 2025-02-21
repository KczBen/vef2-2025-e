pub mod ray {
    use crate::vector3::Vector3;

    #[derive(Default)]
    pub struct Ray {
        origin: Vector3,
        direction: Vector3,
    }

    impl Ray {
        pub fn origin(&self) -> Vector3 {
            self.origin
        }

        pub fn direction(&self)-> Vector3 {
            self.direction
        }

        pub fn new(origin:Vector3, direction: Vector3) -> Ray {
            Ray{
                origin,
                direction,
            }
        }

        pub fn at(&self, t:f32) -> Vector3 {
            return self.origin + t * self.direction
        }
    }
}