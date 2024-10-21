use crate::{
    hit::Hit,
    ray::Ray,
    vec3::{Color, Vec3},
};

use super::{Material, Scatter};

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray_incomming: &Ray, hit: &Hit) -> Option<Scatter> {
        let scatter_dir = hit.normal + Vec3::random_on_unit_sphere();

        let scatter_dir = if scatter_dir.is_near_zero() {
            hit.normal
        } else {
            scatter_dir
        };

        return Some(Scatter {
            attenuation: self.albedo,
            ray: Ray::new(hit.p, scatter_dir),
        });
    }
}

impl Default for Lambertian {
    fn default() -> Self {
        return Self {
            albedo: Color::default(),
        };
    }
}
