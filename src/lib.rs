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
static SETTINGS: OnceLock<shared_mem::SharedMem> = OnceLock::new();

#[wasm_bindgen(start)]
fn init() {
    // Scene
    let mut world = object_list::object_list::ObjectList::default();

    let material_ground = Arc::new(material::Lambertian::new(Vector3::new(0.8, 0.8, 0.0)));
    let object = Arc::new(sphere::sphere::Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, material_ground));
    world.add(object);

    let _ = WORLD.set(Arc::new(RwLock::new(world)));
}

#[wasm_bindgen]
pub async fn init_settings() -> *const shared_mem::SharedMem {
    let settings = shared_mem::SharedMem::default();
    let _ = SETTINGS.set(settings);

    return SETTINGS.get().unwrap();
}

#[wasm_bindgen]
pub fn trace() {
    let mut camera = Camera::new(SETTINGS.get().unwrap());
    camera.location = Vector3::new(-2.0, 2.0, 1.0);
    camera.look_at = Vector3::new(0.0, 0.0, -1.0);

    if let Some(world) = WORLD.get() {
        match world.read() {
            Ok(world) => camera.render(&*world),
            Err(_) => console_log!("Failed to get world")
        }
    };
}

// This is probably all doable without unsafe blocks
#[wasm_bindgen]
pub async unsafe fn get_texture() -> *const u8 {
    return TEXTURE.as_ptr();
}