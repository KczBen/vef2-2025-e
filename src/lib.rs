mod color;
mod ray;
mod scene_object;
mod sphere;
mod object_list;
mod interval;
mod camera;
mod vector_utils;

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
    let binding = sphere::sphere::Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5);
    world.add(&binding);
    let binding = sphere::sphere::Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0);
    world.add(&binding);

    let mut camera = Camera::default();
    camera.render(world);
}

// This is probably all doable without unsafe blocks
#[wasm_bindgen]
pub async unsafe fn get_texture() -> *const u8 {
    return TEXTURE.as_ptr();
}