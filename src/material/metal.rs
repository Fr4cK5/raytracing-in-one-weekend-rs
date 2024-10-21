use crate::{
    hit::Hit,
    ray::Ray,
    vec3::{Color, Vec3},
};

use super::{Material, Scatter};

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray_incomming: &Ray, hit: &Hit) -> Option<Scatter> {
        let reflected = Vec3::reflect(&ray_incomming.direction, &hit.normal);
        let reflected = reflected.norm() + Vec3::random_on_unit_sphere().mul(self.fuzz);

        let scattered = Ray::new(hit.p, reflected);
        if scattered.direction.dot(&hit.normal) <= 0.0 {
            return None;
        }

        return Some(Scatter {
            ray: scattered,
            attenuation: self.albedo,
        });
    }
}

impl Default for Metal {
    fn default() -> Self {
        return Self {
            albedo: Color::default(),
            fuzz: 0.,
        };
    }
}
