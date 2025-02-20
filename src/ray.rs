pub mod ray {
    use nalgebra::Vector3;

    #[derive(Default)]
    pub struct Ray {
        origin: Vector3<f32>,
        direction: Vector3<f32>,
    }

    impl Ray {
        pub fn origin(&self) -> Vector3<f32> {
            self.origin
        }

        pub fn direction(&self)-> Vector3<f32>{
            self.direction
        }

        pub fn new(origin:Vector3<f32>, direction: Vector3<f32>) -> Ray {
            Ray{
                origin,
                direction,
            }
        }

        pub fn at(&self, t:f32) -> Vector3<f32> {
            self.origin + t * self.direction
        }
    }
}