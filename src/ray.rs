use crate::vec3::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        return Ray { origin, direction };
    }
    pub fn at(&self, t: f64) -> Point3 {
        return self.origin + self.direction.mul(t);
    }
}
