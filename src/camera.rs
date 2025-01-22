use nalgebra::Vector3;
use rand::Rng;

use crate::{color, interval, object_list::object_list::ObjectList, ray::ray::Ray, scene_object::scene_object::{HitRecord, SceneObject}, TEXTURE};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u64,
    pub image_height: u64,
    pub samples_per_pixel: u64,

    pixel_samples_scale:f64,
    camera_centre: Vector3<f64>,
    pixel_00_loc: Vector3<f64>,
    pixel_delta_u: Vector3<f64>, 
    pixel_delta_v: Vector3<f64>, 
    texture: Vec<u8>, 
}

impl Camera {
    pub fn render(&mut self, world: ObjectList) {
        self.initialise();

        for row in 0..self.image_height {
            for col in 0..self.image_width {
                let pixel_center = self.pixel_00_loc + (self.pixel_delta_u.component_mul(&Vector3::from_element(col as f64)))
                                    + (self.pixel_delta_v.component_mul(&Vector3::from_element(row as f64)));
    
                let mut pixel_color = Vector3::new(0.0, 0.0, 0.0);

                for _sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(col, row);
                    pixel_color += Self::ray_color(&ray, &world);
                }

                color::write_color(&(self.pixel_samples_scale * &pixel_color), &mut self.texture, ((self.image_width * ((self.image_height - 1) - row) + col) * 3) as usize);
            }
        }
    
        // yolo
        unsafe {
            TEXTURE = self.texture.clone();
        }
    }

    fn get_ray(&self, i:u64, j:u64) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel_00_loc + ((i as f64 + offset.x) * self.pixel_delta_u)
                            + ((j as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = self.camera_centre;
        let ray_direction = pixel_sample - ray_origin;

        return Ray::new(ray_origin, ray_direction);
    }

    fn sample_square() -> Vector3<f64> {
        // return Vector3::new(0.5, 0.5, 0.0);
        return Vector3::new(fastrand::f64() - 0.5, fastrand::f64() - 0.5, 0.0);
    }

    fn initialise(&mut self) {
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * ((self.image_width as f64)/self.image_height as f64);
        self.camera_centre = Vector3::new(0.0, 0.0, 0.0);

        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);
        
        self.pixel_delta_u = viewport_u.component_div(&Vector3::from_element(self.image_width as f64));
        self.pixel_delta_v = viewport_v.component_div(&Vector3::from_element(self.image_height as f64));

        let viewport_upper_left = self.camera_centre - Vector3::new(0.0, 0.0, focal_length)
                                    - viewport_u.component_div(&Vector3::from_element(2.0)) - viewport_v.component_div(&Vector3::from_element(2.0));

        self.pixel_00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);        
    }

    fn ray_color(ray: &Ray, world: &ObjectList) -> Vector3<f64> {
        let mut record = HitRecord::default();

        if world.hit(ray, interval::Interval::new(0.0, std::f64::INFINITY), &mut record) {
            return 0.5 * (record.normal + Vector3::new(1.0, 1.0, 1.0));
        }

        let unit_direction = nalgebra::UnitVector3::new_normalize(ray.direction());
        let a = 0.5*unit_direction.y + 1.0;
        return (1.0-a)*Vector3::new(1.0, 1.0, 1.0) + a*Vector3::new(0.5, 0.7, 1.0);
    }
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0/9.0;
        let image_height = 1440;
        let image_width = (aspect_ratio * image_height as f64) as u64;
        let samples_per_pixel = 4;
        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        return Camera{
            samples_per_pixel,
            aspect_ratio,
            image_width,
            image_height,
            texture: vec![0u8; image_width as usize * image_height as usize * 3],
            camera_centre: Vector3::new(0.0, 0.0, 0.0),
            pixel_00_loc: Vector3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vector3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vector3::new(0.0, 0.0, 0.0),
            pixel_samples_scale
        }
    }
}