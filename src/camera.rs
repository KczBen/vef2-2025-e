use nalgebra::Vector3;

use crate::{color, interval, object_list::object_list::ObjectList, ray::ray::Ray, scene_object::scene_object::SceneObject, shared_mem::SharedMem, TEXTURE};

pub struct Camera {
    #[allow(dead_code)]
    pub aspect_ratio: f32,
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,

    pub fov_vertical: f32,
    pub location: Vector3<f32>,
    pub look_at: Vector3<f32>,
    pub up: Vector3<f32>,

    pixel_samples_scale:f32,
    camera_centre: Vector3<f32>,
    pixel_00_loc: Vector3<f32>,
    pixel_delta_u: Vector3<f32>, 
    pixel_delta_v: Vector3<f32>,
    u: Vector3<f32>,
    v: Vector3<f32>,
    w: Vector3<f32>,    
    texture: Vec<u8>, 
}

impl Camera {
    pub fn new(settings: &SharedMem) -> Self {
        return Self {
            aspect_ratio: settings.target_width as f32 / settings.target_height as f32,
            image_width: settings.target_width,
            image_height: settings.target_height,
            samples_per_pixel: settings.samples_per_pixel,
            max_depth: settings.max_bounces,
            ..Default::default()
        }
    }

    pub fn set_resolution(&mut self, width: u32, height: u32) {
        self.image_width = width;
        self.image_height = height;
        self.aspect_ratio = width as f32 / height as f32;
    }
    
    pub fn render(&mut self, world: &ObjectList) {
        self.initialise();

        for row in 0..self.image_height {
            for col in 0..self.image_width {
                let mut pixel_color = Vector3::new(0.0, 0.0, 0.0);

                for _sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(col, row);
                    pixel_color += Self::ray_color(&ray, &world, self.max_depth);
                }

                color::write_color(&(self.pixel_samples_scale * &pixel_color), &mut self.texture, ((self.image_width * ((self.image_height - 1) - row) + col) * 3) as usize);
            }
        }
    
        // yolo
        unsafe {
            TEXTURE = self.texture.clone();
        }
    }

    fn get_ray(&self, i:u32, j:u32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel_00_loc + ((i as f32 + offset.x) * self.pixel_delta_u)
                            + ((j as f32 + offset.y) * self.pixel_delta_v);

        let ray_origin = self.camera_centre;
        let ray_direction = (pixel_sample - ray_origin).normalize();

        return Ray::new(ray_origin, ray_direction);
    }

    fn sample_square() -> Vector3<f32> {
        // return Vector3::new(0.5, 0.5, 0.0);
        return Vector3::new(fastrand::f32() - 0.5, fastrand::f32() - 0.5, 0.0);
    }

    fn initialise(&mut self) {
        self.camera_centre = self.location;
        let focal_length = (self.location - self.look_at).norm();
        let theta = self.fov_vertical.to_radians();
        let h = f32::tan(theta/2.0);

        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * ((self.image_width as f32)/self.image_height as f32);

        self.w = (self.location - self.look_at).normalize();
        self.u = (self.up.cross(&self.w)).normalize();
        self.v = self.w.cross(&self.u);

        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;
        
        self.pixel_delta_u = viewport_u.component_div(&Vector3::from_element(self.image_width as f32));
        self.pixel_delta_v = viewport_v.component_div(&Vector3::from_element(self.image_height as f32));

        let viewport_upper_left = self.camera_centre - (focal_length * self.w) - viewport_u/2.0 - viewport_v/2.0; 
        self.pixel_00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);        
    }

    fn ray_color(ray: &Ray, world: &ObjectList, depth: u32) -> Vector3<f32> {
        if depth <= 0 {
            return Vector3::new(0.0, 0.0, 0.0);
        }

        if let Some(hit) = world.hit(ray, interval::Interval::new(0.001, std::f32::INFINITY)) {
            // let direction = hit.normal + vector_utils::random_vec3_unit();
            // return 0.5 * Self::ray_color(&Ray::new(hit.point, direction), world, depth - 1);
            let mut scattered: Ray = Ray::default();
            let mut attenuation = Vector3::default();

            if hit.material.scatter(ray, &hit, &mut attenuation, &mut scattered) {
                return Self::ray_color(&scattered, world, depth-1).component_mul(&attenuation);
            }
        }

        let a = 0.5*ray.direction().y + 1.0;
        return (1.0-a)*Vector3::new(1.0, 1.0, 1.0) + a*Vector3::new(0.5, 0.7, 1.0);
    }
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0/9.0;
        let image_height = 1440;
        let image_width = (aspect_ratio * image_height as f32) as u32;
        let samples_per_pixel = 4;
        let pixel_samples_scale = 1.0 / samples_per_pixel as f32;

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
            pixel_samples_scale,
            max_depth: 8,
            fov_vertical: 90.0,
            location: Vector3::new(0.0, 0.0, 0.0),
            look_at: Vector3::new(0.0, 0.0, -1.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            u: Vector3::new(0.0, 0.0, 0.0),
            v: Vector3::new(0.0, 0.0, 0.0),
            w: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}