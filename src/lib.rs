mod color;
mod ray;
mod scene_object;
mod sphere;
mod object_list;
mod interval;
mod camera;
mod vector_utils;
mod material;
mod vector3;

use std::sync::Arc;

use camera::Camera;
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

static mut TEXTURE:Vec<u8> = Vec::new();

#[wasm_bindgen(start)]
fn main() {
    // Scene
    let mut world = object_list::object_list::ObjectList::default();

    let material_ground = Arc::new(material::Lambertian::new(Vector3::new(0.8,0.8,0.0)));
    let material_center = Arc::new(material::Lambertian::new(Vector3::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(material::Dielectric::new(1.5));
    let material_bubble = Arc::new(material::Dielectric::new(1.0 / 1.5));
    let material_right = Arc::new(material::Metal::new(Vector3::new(0.8, 0.6, 0.2), 1.0));
    
    let object_binding = sphere::sphere::Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, material_ground);
    world.add(&object_binding);
    let object_binding = sphere::sphere::Sphere::new(Vector3::new(0.0, 0.0, -1.2), 0.5, material_center);
    world.add(&object_binding);
    let object_binding = sphere::sphere::Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5, material_left);
    world.add(&object_binding);
    let object_binding = sphere::sphere::Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.4, material_bubble);
    world.add(&object_binding);
    let object_binding = sphere::sphere::Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, material_right);
    world.add(&object_binding);

    let mut camera = Camera::default();
    camera.location = Vector3::new(-2.0, 2.0, 1.0);
    camera.look_at = Vector3::new(0.0, 0.0, -1.0);
    camera.render(world);
}

// This is probably all doable without unsafe blocks
#[wasm_bindgen]
pub async unsafe fn get_texture() -> *const u8 {
    return TEXTURE.as_ptr();
}