use crate::{ray::Ray, vec3::{Point3, Vec3}};

pub struct Hit {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl Hit {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            outward_normal.inv()
        }
    }
}
impl Default for Hit {
    fn default() -> Self {
        return Self {
            p: Point3::default(),
            normal: Vec3::default(),
            t: 0.,
            front_face: true,
        };
    }
}
