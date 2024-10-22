#![allow(dead_code)]

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::utils;

#[derive(Debug, Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);
pub type Point3 = Vec3;
pub type Color = Vec3;

impl Default for Vec3 {
    fn default() -> Self {
        return Self(0., 0., 0.);
    }
}

impl Vec3 {
    pub fn len(&self) -> f64 {
        return self.len_squared().sqrt();
    }
    pub fn len_squared(&self) -> f64 {
        return self.0 * self.0 + self.1 * self.1 + self.2 * self.2;
    }
    pub fn inv(&self) -> Self {
        return Self(-self.0, -self.1, -self.2);
    }
    pub fn inv_mut(&mut self) {
        self.0 = -self.0;
        self.1 = -self.1;
        self.2 = -self.2;
    }

    #[inline]
    pub fn x(&self) -> f64 {
        return self.0;
    }
    #[inline]
    pub fn y(&self) -> f64 {
        return self.1;
    }
    #[inline]
    pub fn z(&self) -> f64 {
        return self.2;
    }

    pub fn mul(&self, x: f64) -> Self {
        return Self(self.0 * x, self.1 * x, self.2 * x);
    }
    pub fn div(&self, x: f64) -> Self {
        return Self(self.0 / x, self.1 / x, self.2 / x);
    }
    pub fn add_scalar(&self, x: f64) -> Self {
        return Self(self.0 + x, self.1 + x, self.2 + x);
    }
    pub fn sub_scalar(&self, x: f64) -> Self {
        return Self(self.0 - x, self.1 - x, self.2 - x);
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        return self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2;
    }
    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        return Vec3(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        );
    }
    pub fn norm(&self) -> Vec3 {
        let len = self.len();
        return Vec3(self.0 / len, self.1 / len, self.2 / len);
    }

    pub fn is_near_zero(&self) -> bool {
        let t = 1e-8;
        return self.0.abs() < t && self.1.abs() < t && self.2.abs() < t;
    }

    pub fn from_tup(base: (f64, f64, f64)) -> Vec3 {
        return Vec3(base.0, base.1, base.2);
    }

    pub fn from_floats(a: f64, b: f64, c: f64) -> Vec3 {
        return Vec3(a, b, c);
    }

    pub fn random() -> Vec3 {
        return Vec3(
            utils::rand_float(),
            utils::rand_float(),
            utils::rand_float(),
        );
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        return Vec3(
            utils::rand_float_range(min, max),
            utils::rand_float_range(min, max),
            utils::rand_float_range(min, max),
        );
    }

    pub fn random_on_unit_sphere() -> Vec3 {
        loop {
            let v = Self::random_range(-1., 1.);
            let len_squared = v.len_squared();
            // Small floating point hole in the center can underflow to 0
            if 1e-160 < len_squared && len_squared <= 1. {
                return v.norm();
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let mut on_unit_sphere = Self::random_on_unit_sphere();
        if on_unit_sphere.dot(normal) <= 0. {
            on_unit_sphere.inv_mut();
        }
        return on_unit_sphere;
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        return *v - n.mul(v.dot(n) * 2.);
    }

    pub fn refract(uv: &Vec3, n: &Vec3, refraction_index: f64) -> Vec3 {
        let cos_theta = uv.inv().dot(n).min(1.);
        let out_perp = *uv + n.mul(cos_theta);
        let out_perp = (&out_perp).mul(refraction_index);
        let out_par = n.mul(-(1. - out_perp.len_squared()).abs().sqrt());
        return out_perp + out_par;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        return Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2);
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        return Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2);
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        return Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2);
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Self) -> Self::Output {
        return Self(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2);
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
    }
}
