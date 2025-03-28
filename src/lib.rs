mod color;
mod ray;
mod scene_object;
mod sphere;
mod object_list;
mod interval;
mod camera;
pub mod vector_utils;
mod material;
pub mod vector3;
mod rng;
mod shared_mem;

use std::cell::RefCell;
use std::sync::{Arc, OnceLock, RwLock};

use camera::Camera;
use wasm_bindgen::prelude::*;
use crate::vector3::Vector3;
use crate::rng::Xorshift32State;

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

// Maybe multi-threading later
thread_local! {
    static RNG: RefCell<Xorshift32State> = RefCell::new(Xorshift32State::new(0xBAD5EED));
}

static WORLD: OnceLock<Arc<RwLock<object_list::object_list::ObjectList>>> = OnceLock::new();
static SETTINGS: OnceLock<RwLock<shared_mem::SharedMem>> = OnceLock::new();

#[wasm_bindgen(start)]
fn init() {
    // Scene
    let world = object_list::object_list::ObjectList::default();
    let _ = WORLD.set(Arc::new(RwLock::new(world)));
}

#[wasm_bindgen]
pub async fn init_settings() -> *const shared_mem::SharedMem {
    let settings = shared_mem::SharedMem::default();
    let _ = SETTINGS.set(RwLock::new(settings));

    return SETTINGS.get().unwrap().write().as_deref().unwrap();
}

#[wasm_bindgen]
pub async fn trace() {
    let mut camera = Camera::new(SETTINGS.get().unwrap().read().as_ref().unwrap());
    
    if let Some(world) = WORLD.get() {
        match world.read() {
            Ok(world) => camera.render(&*world).await,
            Err(_) => console_log!("Failed to get world")
        }
    };
}

#[wasm_bindgen]
pub fn add_sphere(x: f32, y: f32, z: f32, diameter: f32, material: u32, r: f32, g: f32, b: f32, special: f32) {
    if let Some(world) = WORLD.get() {
        match world.write() {
            Ok(mut world) => {
                // Metal 
                if material == 1 {
                    let mat = Arc::new(material::Metal::new(Vector3::new(r, g, b), special));
                    let object = Arc::new(sphere::sphere::Sphere::new(Vector3::new(x, y, z), diameter, mat));
                    world.add(object);
                }
                // Dielectric
                else if material == 2 {
                    let mat = Arc::new(material::Dielectric::new(special));
                    let object = Arc::new(sphere::sphere::Sphere::new(Vector3::new(x, y, z), diameter, mat));
                    world.add(object);
                }
                // Emissive Lambertian
                else if material == 3 {
                    let mat = Arc::new(material::Emissive::new(Vector3::new(r, g, b), special));
                    let object = Arc::new(sphere::sphere::Sphere::new(Vector3::new(x, y, z), diameter, mat));
                    world.add(object);
                }
                // Default to Lambertian
                else {
                    let mat = Arc::new(material::Lambertian::new(Vector3::new(r, g, b)));
                    let object = Arc::new(sphere::sphere::Sphere::new(Vector3::new(x, y, z), diameter, mat));
                    world.add(object);
                } 
            },

            Err(_) => console_log!("Failed to get write lock on world")
        }
    }
}

// This is probably all doable without unsafe blocks
#[wasm_bindgen]
pub async unsafe fn get_texture() -> *const u8 {
    return TEXTURE.as_ptr();
}