pub mod ray {
    use nalgebra::Vector3;

    #[derive(Default)]
    pub struct Ray {
        origin: Vector3<f64>,
        direction: Vector3<f64>,
    }

    impl Ray {
        pub fn origin(&self) -> Vector3<f64> {
            self.origin
        }

        pub fn direction(&self)-> Vector3<f64>{
            self.direction
        }

        pub fn new(origin:Vector3<f64>, direction: Vector3<f64>) -> Ray {
            Ray{
                origin,
                direction,
            }
        }

        pub fn at(&self, t:f64) -> Vector3<f64> {
            self.origin + t * self.direction
        }
    }
}