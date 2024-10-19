use crate::{hit::Hit, ray::Ray, sphere::Sphere};

pub type World = Vec<Sphere>;

pub trait AnyHit {
    fn any_hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<Hit>;
}

impl AnyHit for World {
    fn any_hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<Hit> {
        let mut hit = Hit::default();
        let mut has_hit = false;
        let mut closest = tmax;

        for item in self.iter() {
            if item.hit(r, tmin, closest, &mut hit) {
                has_hit = true;
                closest = hit.t;
            }
        }

        return if has_hit { Some(hit) } else { None };
    }
}
