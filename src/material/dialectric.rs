use crate::{
    hit::Hit,
    ray::Ray,
    utils,
    vec3::{Color, Vec3},
};

use super::{Material, Scatter};

pub struct Dialectric {
    pub refrecation_index: f64,
}

impl Dialectric {
    fn reflectance(cos: f64, refraction_index: f64) -> f64 {
        let r0 = (1. - refraction_index) / (1. + refraction_index);
        let r0 = r0 * r0;
        return r0 + (1. - r0) * (1. - cos).powf(5.);
    }
}

impl Material for Dialectric {
    fn scatter(&self, ray_incomming: &Ray, hit: &Hit) -> Option<Scatter> {
        let attenuation = Color::from_floats(1., 1., 1.);
        let refraction_index = if hit.front_face {
            1. / self.refrecation_index
        } else {
            self.refrecation_index
        };

        let unit_dir = ray_incomming.direction.norm();

        let cos_theta = unit_dir.inv().dot(&hit.normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let refracted = if refraction_index * sin_theta > 1.
            || Self::reflectance(cos_theta, refraction_index) > utils::rand_float()
        {
            Vec3::reflect(&unit_dir, &hit.normal)
        } else {
            Vec3::refract(&unit_dir, &hit.normal, refraction_index)
        };

        return Some(Scatter {
            attenuation,
            ray: Ray::new(hit.p, refracted),
        });
    }
}
