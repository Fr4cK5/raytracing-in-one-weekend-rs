use std::sync::{Arc, Mutex};

use crate::{interval::Interval, vec3::Color};

pub fn write_color(out: Arc<Mutex<Vec<String>>>, col: &Color, x: usize, y: usize, width: usize) {
    let intensity = Interval::new(0., 0.99999);

    let line = format!(
        "{} {} {}",
        (256f64 * intensity.clamp(col.0.to_gamma())) as u8,
        (256f64 * intensity.clamp(col.1.to_gamma())) as u8,
        (256f64 * intensity.clamp(col.2.to_gamma())) as u8,
    );
    let mut handle = out.lock().unwrap();
    handle[width * y + x] = line;
}

#[inline]
pub fn rand_float() -> f64 {
    return fastrand::f64();
}

#[inline]
pub fn rand_float_range(min: f64, max: f64) -> f64 {
    return min + (max - min) * fastrand::f64();
}

pub trait ToGamma {
    fn to_gamma(self) -> f64;
}

impl ToGamma for f64 {
    #[inline]
    fn to_gamma(self) -> f64 {
        if self > 0. {
            return self.sqrt();
        }

        return 0.;
    }
}
