#![allow(dead_code)]

use core::f64;

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        return Self { min, max };
    }

    pub fn size(&self) -> f64 {
        return self.max - self.min;
    }

    pub fn contains(&self, x: f64) -> bool {
        return self.min <= x && x <= self.max;
    }

    pub fn sourrounds(&self, x: f64) -> bool {
        return self.min < x && x < self.max;
    }

    pub const fn empty() -> Self {
        return Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        };
    }

    pub const fn universe() -> Self {
        return Self {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        };
    }
}

impl Default for Interval {
    fn default() -> Self {
        return Self::empty();
    }
}
