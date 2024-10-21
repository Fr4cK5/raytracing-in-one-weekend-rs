use std::sync::Arc;

use crate::{hit::Hit, interval::Interval, ray::Ray, sphere::Sphere};

pub type World = Vec<Arc<Sphere>>;

pub trait AnyHit {
    fn any_hit(&self, r: &Ray, interval: Interval) -> Option<Hit>;
}

impl AnyHit for World {
    fn any_hit(&self, r: &Ray, mut interval: Interval) -> Option<Hit> {
        let mut hit = Hit::default();
        let mut has_hit = false;

        for item in self.iter() {
            if item.hit(r, &interval, &mut hit) {
                has_hit = true;
                interval.max = hit.t;
            }
        }

        return if has_hit { Some(hit) } else { None };
    }
}
