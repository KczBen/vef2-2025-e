use nalgebra::Vector3;

// I would prefer a BSDF but this *is* simpler as it is in the book
use crate::{ray::ray::Ray, scene_object::scene_object::HitRecord, vector_utils::{self, near_zero, random_vec3_unit}};

pub trait Material {
    fn scatter(&self, incoming_ray: &Ray, hit_record: &HitRecord, attenuation: &mut Vector3<f64>, scattered_ray: &mut Ray) -> bool;
}

#[derive(Default)]
pub struct Lambertian {
    albedo: Vector3<f64>
}

impl Lambertian {
    pub fn new(albedo: Vector3<f64>) -> Self{
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
    albedo: Vector3<f64>
}

impl Metal {
    pub fn new(albedo: Vector3<f64>) -> Self{
        return Self { albedo };
    }
}

impl Material for Metal {
    fn scatter(&self, incoming_ray: &Ray, hit_record: &HitRecord, attenuation: &mut Vector3<f64>, scattered_ray: &mut Ray) -> bool {
        let reflection_direction = vector_utils::reflect(incoming_ray.direction(), hit_record.normal);
        *scattered_ray = Ray::new(hit_record.point, reflection_direction);
        *attenuation = self.albedo;

        return true;
    }
}