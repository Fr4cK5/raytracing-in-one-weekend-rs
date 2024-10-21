use crate::{hit::Hit, ray::Ray, vec3::Color};

pub mod lambertian;
pub mod metal;

pub struct Scatter {
    pub attenuation: Color,
    pub ray: Ray,
}

pub trait Material {
    fn scatter(&self, ray_incomming: &Ray, hit: &Hit) -> Option<Scatter>;
}
