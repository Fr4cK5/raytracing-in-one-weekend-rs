// https://raytracing.github.io/books/RayTracingInOneWeekend.html

#![allow(clippy::needless_return)]

use core::f64;
use std::{fs, time::Instant};

use camera::Camera;
use interval::Interval;
use ray::Ray;
use vec3::{Color, Point3, Vec3};
use world::{AnyHit, World};

mod camera;
mod color_utils;
mod hit;
mod interval;
mod ray;
mod sphere;
mod vec3;
mod world;

fn main() {
    // Scene
    let mut world = World::new();
    world.push(sphere::Sphere {
        center: Vec3::from_floats(0., 0., -1.),
        radius: 0.5,
    });
    world.push(sphere::Sphere {
        center: Vec3::from_floats(0., -100.5, -1.),
        radius: 100.,
    });

    let mut cam = Camera::default();
    cam.aspect_ratio = 16. / 9.;
    cam.img_width = 400;

    cam.render(&world);
}

// pub fn ray_color(r: &Ray, world: &World) -> Color {
//     if let Some(hit) = world.any_hit(r, Interval::new(0., f64::INFINITY)) {
//         return (hit.normal + Color::from_floats(1., 1., 1.)).mul(0.5);
//     }
//
//     let unit_dir = r.direction.norm();
//     let a = (unit_dir.y() + 1.) * 0.5;
//     return Color::from_floats(1., 1., 1.).mul(1. - a) + Color::from_floats(0.5, 0.7, 1.).mul(a);
// }

pub fn sphere_hit(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = *center - r.origin;
    let a = r.direction.len_squared();
    let h = r.direction.dot(&oc);
    let c = oc.len_squared() - radius * radius;

    // Woudl call this "discriminant", but the LSP suggestions suck
    let disc = h * h - a * c;

    if disc < 0. {
        return -1.;
    } else {
        return (h - disc.sqrt()) / a;
    }
}
