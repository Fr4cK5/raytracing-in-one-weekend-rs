#![allow(dead_code)]

use std::sync::Arc;

use crate::{
    hit::Hit,
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material>) -> Self {
        return Self {
            center,
            radius,
            material,
        };
    }

    pub fn hit(&self, r: &Ray, interval: &Interval, hit: &mut Hit) -> bool {
        let oc = self.center - r.origin;
        let a = r.direction.len_squared();
        let h = r.direction.dot(&oc);
        let c = oc.len_squared() - self.radius * self.radius;
        // discriminant, but the LSP suggestions suck
        let disc: f64 = h * h - a * c;

        if disc < 0. {
            return false;
        }

        let disc_sqrt = disc.sqrt();

        let mut root = (h - disc_sqrt) / a;
        if !interval.sourrounds(root) {
            root = (h + disc_sqrt) / a;
            if !interval.sourrounds(root) {
                return false;
            }
        }

        hit.t = root;
        hit.p = r.at(hit.t);
        let outward_normal = (hit.p - self.center).div(self.radius);
        hit.set_face_normal(r, &outward_normal);
        hit.material = Some(Arc::clone(&self.material));

        return true;
    }
}
