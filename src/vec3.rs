#![allow(dead_code)]

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

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

    pub fn x(&self) -> f64 {
        return self.0;
    }
    pub fn y(&self) -> f64 {
        return self.1;
    }
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
        return Vec3(
            self.0 / len,
            self.1 / len,
            self.2 / len,
        );
    }

    pub fn from_tup(base: (f64, f64, f64)) -> Vec3 {
        return Vec3(base.0, base.1, base.2);
    }

    pub fn from_floats(a: f64, b: f64, c: f64) -> Vec3 {
        return Vec3(a, b, c);
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
        return Vec3(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
        );
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
        return Self(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
        );
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        return Self(
            self.0 * rhs.0,
            self.1 * rhs.1,
            self.2 * rhs.2,
        );
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
        return Self(
            self.0 / rhs.0,
            self.1 / rhs.1,
            self.2 / rhs.2,
        );
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
    }
}
