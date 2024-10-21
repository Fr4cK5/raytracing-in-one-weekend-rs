// https://raytracing.github.io/books/RayTracingInOneWeekend.html

#![allow(clippy::needless_return)]

use core::f64;
use std::rc::Rc;

use camera::Camera;
use material::{lambertian::Lambertian, metal::Metal, Material};
use ray::Ray;
use vec3::{Point3, Vec3};
use world::World;

mod camera;
mod hit;
mod interval;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;
mod world;

fn main() {

    let mat_ground: Rc<dyn Material> = Rc::new(Lambertian {
        albedo: Vec3(0.4, 0.6, 0.1),
    });

    let mat_left: Rc<dyn Material> = Rc::new(Metal {
        albedo: Vec3(0.694, 0.384, 0.525), // rgb(177, 98, 134)
        fuzz: 0.0,
    });
    let mat_center: Rc<dyn Material> = Rc::new(Lambertian {
        albedo: Vec3(0.5, 0.1, 0.9),
    });
    let mat_right: Rc<dyn Material> = Rc::new(Metal {
        albedo: Vec3(1., 0.6, 0.),
        fuzz: 0.3,
    });

    // Scene
    let mut world = World::new();
    world.push(sphere::Sphere {
        center: Vec3::from_floats(0., -100.5, -1.),
        radius: 100.,
        material: Rc::clone(&mat_ground),
    });

    world.push(sphere::Sphere {
        center: Vec3::from_floats(-1., 0., -1.),
        radius: 0.5,
        material: Rc::clone(&mat_left),
    });
    world.push(sphere::Sphere {
        center: Vec3::from_floats(0., 0., -1.2),
        radius: 0.5,
        material: Rc::clone(&mat_center),
    });
    world.push(sphere::Sphere {
        center: Vec3::from_floats(1., 0., -1.),
        radius: 0.5,
        material: Rc::clone(&mat_right),
    });

    let mut cam = Camera::default();
    cam.aspect_ratio = 16. / 9.;
    cam.img_width = 400;            // 2560
    cam.samples_per_pixel = 500;    // 1000
    cam.max_bounces_per_ray = 50;   // 100

    cam.render(&world);
}

pub fn sphere_hit(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = *center - r.origin;
    let a = r.direction.len_squared();
    let h = r.direction.dot(&oc);
    let c = oc.len_squared() - radius * radius;

    // Would call this "discriminant", but the LSP suggestions suck
    let disc = h * h - a * c;

    if disc < 0. {
        return -1.;
    } else {
        return (h - disc.sqrt()) / a;
    }
}
