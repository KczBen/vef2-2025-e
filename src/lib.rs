mod color;
mod ray;

use wasm_bindgen::prelude::*;
use nalgebra::Vector3;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// println, basically
#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[allow(unused_imports)]
pub(crate) use console_log;

fn ray_color(ray:&ray::ray::Ray) -> Vector3<f64> {
    let t = hit_sphere(Vector3::new(0.0, 0.0, -1.0), 0.5, ray);

    if t > 0.0 {
        let normal = nalgebra::UnitVector3::new_normalize(ray.at(t) - Vector3::new(0.0, 0.0, -1.0));
        return 0.5 * Vector3::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0);
    }

    let unit_direction = nalgebra::UnitVector3::new_normalize(ray.direction());
    let a = 0.5*unit_direction.y + 1.0;
    return (1.0-a)*Vector3::new(1.0, 1.0, 1.0) + a*Vector3::new(0.5, 0.7, 1.0);
}

fn hit_sphere(sphere_centre:Vector3<f64>, radius:f64, ray:&ray::ray::Ray) -> f64 {
    let oc = sphere_centre - ray.origin();
    let a = ray.direction().norm().powi(2);
    let h = nalgebra::Vector::dot(&ray.direction(), &oc);
    let c = oc.norm().powi(2) - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (h - discriminant.sqrt() ) / a;
    }
}

static mut TEXTURE:Vec<u8> = Vec::new();

#[wasm_bindgen(start)]
fn main() {
    // Image setup
    let aspect_ratio = 16.0/9.0;

    // get width from height to line up with common notations i.e. 1080p
    let image_height = 1440;
    let image_width = (aspect_ratio * image_height as f64) as u64;

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ((image_width as f64)/image_height as f64);
    let camera_centre = Vector3::new(0.0, 0.0, 0.0);

    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);
    
    let pixel_delta_u = viewport_u.component_div(&Vector3::from_element(image_width as f64));
    let pixel_delta_v = viewport_v.component_div(&Vector3::from_element(image_height as f64));

    let viewport_upper_left = camera_centre - Vector3::new(0.0, 0.0, focal_length)
                                - viewport_u.component_div(&Vector3::from_element(2.0)) - viewport_v.component_div(&Vector3::from_element(2.0));

    let pixel_00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    
    let mut texture = vec![0u8; image_width as usize * image_height as usize * 3];
    for row in 0..image_height {
        for col in 0..image_width {
            let pixel_center = pixel_00_loc + (pixel_delta_u.component_mul(&Vector3::from_element(col as f64)))
                                + (pixel_delta_v.component_mul(&Vector3::from_element(row as f64)));
            let ray_direction = pixel_center - camera_centre;
            let ray = ray::ray::Ray::new(camera_centre, ray_direction);

            let pixel_color = ray_color(&ray);

            // Write pixels from the bottom left, left to right, bottom to top
            // Needed for WebGL texture sampling later
            color::write_color(&pixel_color, &mut texture, ((image_width * ((image_height - 1) - row) + col) * 3) as usize);
        }
    }

    // yolo
    unsafe {
        TEXTURE = texture;
    }
}

// This is probably all doable without unsafe blocks
#[wasm_bindgen]
pub async unsafe fn get_texture() -> *const u8 {
    return TEXTURE.as_ptr();
}