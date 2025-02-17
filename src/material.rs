use nalgebra::Vector3;
use wasm_bindgen_futures::js_sys::Math::cos;

// I would prefer a BSDF but this *is* simpler as it is in the book
use crate::{ray::ray::Ray, scene_object::scene_object::HitRecord, vector_utils::{self, near_zero, random_vec3_sphere, random_vec3_unit, reflect, refract}};

pub trait Material {
    fn scatter(&self, incoming_ray: &Ray, hit_record: &HitRecord, attenuation: &mut Vector3<f64>, scattered_ray: &mut Ray) -> bool;
}

#[derive(Default)]
pub struct Lambertian {
    albedo: Vector3<f64>
}

impl Lambertian {
    pub fn new(albedo: Vector3<f64>) -> Self {
        return Self { albedo };
    }
}

impl Material for Lambertian {
    fn scatter(&self, incoming_ray: &Ray, hit_record: &HitRecord, attenuation: &mut Vector3<f64>, scattered_ray: &mut Ray) -> bool {
        let mut scatter_dicretion = hit_record.normal + random_vec3_unit();

        if near_zero(scatter_dicretion) {
            scatter_dicretion = hit_record.normal;
        }

        *scattered_ray = Ray::new(hit_record.point, scatter_dicretion);
        *attenuation = self.albedo;

        return true;
    }
}

#[derive(Default)]
pub struct Metal {
    albedo: Vector3<f64>,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Vector3<f64>, fuzz: f64) -> Self {
        // equivalent of fuzz < 1 ? fuzz : 1.0
        return Self { albedo, fuzz: if fuzz < 1.0 { fuzz } else { 1.0 } };
    }
}

impl Material for Metal {
    fn scatter(&self, incoming_ray: &Ray, hit_record: &HitRecord, attenuation: &mut Vector3<f64>, scattered_ray: &mut Ray) -> bool {
        let mut reflection_direction = vector_utils::reflect(&incoming_ray.direction(), &hit_record.normal);
        reflection_direction = reflection_direction.normalize() + (self.fuzz * random_vec3_sphere());
        *scattered_ray = Ray::new(hit_record.point, reflection_direction);
        *attenuation = self.albedo;

        return scattered_ray.direction().dot(&hit_record.normal) > 0.0;
    }
}

#[derive(Default)]
pub struct Dielectric {
    refraction_index: f64
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        return Self { refraction_index };
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 *= r0;

        return r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5);
    }
}

impl Material for Dielectric {
    fn scatter(&self, incoming_ray: &Ray, hit_record: &HitRecord, attenuation: &mut Vector3<f64>, scattered_ray: &mut Ray) -> bool {
        // Transmission colour
        *attenuation = Vector3::new(1.0, 1.0, 1.0);

        let ri = if hit_record.front_face {1.0/self.refraction_index} else {self.refraction_index};

        let unit_direction = incoming_ray.direction().normalize();
        let cos_theta = f64::min(-unit_direction.dot(&hit_record.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = ri * sin_theta > 1.0;
        let direction;

        if cannot_refract || Self::reflectance(cos_theta, ri) > fastrand::f64() {
            direction = reflect(&unit_direction, &hit_record.normal);
        }

        else {
            direction = refract(&unit_direction, &hit_record.normal, ri);
        }
        
        *scattered_ray = Ray::new(hit_record.point, direction);

        return true;
    }
}