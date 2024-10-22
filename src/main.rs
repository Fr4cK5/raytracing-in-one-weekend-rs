// https://raytracing.github.io/books/RayTracingInOneWeekend.html

#![allow(clippy::needless_return)]

use core::f64;
use std::sync::Arc;

use camera::Camera;
use material::{dialectric::Dialectric, lambertian::Lambertian, metal::Metal, Material};
use ray::Ray;
use sphere::Sphere;
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
    let mat_ground: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: Vec3(0.4, 0.6, 0.1),
    });

    let mat_left: Arc<dyn Material> = Arc::new(Metal {
        albedo: Vec3(0.694, 0.384, 0.525), // rgb(177, 98, 134)
        fuzz: 0.0,
    });
    let mat_center: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: Vec3(0.5, 0.1, 0.9),
    });
    let mat_center_glass: Arc<dyn Material> = Arc::new(Dialectric {
        refrecation_index: 1. / 1.33,
    });
    let mat_right: Arc<dyn Material> = Arc::new(Metal {
        albedo: Vec3(1., 0.6, 0.),
        fuzz: 0.3,
    });

    // Scene
    let mut world = World::new();
    world.push(Sphere {
        center: Vec3::from_floats(0., -100.5, -1.),
        radius: 100.,
        material: Arc::clone(&mat_ground),
    });

    world.push(Sphere {
        center: Vec3::from_floats(-1., 0., -1.),
        radius: 0.5,
        material: Arc::clone(&mat_left),
    });
    world.push(Sphere {
        center: Vec3::from_floats(0., 0., -2.4),
        radius: 0.5,
        material: Arc::clone(&mat_center),
    });
    world.push(Sphere {
        center: Vec3::from_floats(0., 0., -1.2),
        radius: 0.5,
        material: Arc::clone(&mat_center_glass),
    });
    world.push(Sphere {
        center: Vec3::from_floats(1., 0., -1.),
        radius: 0.5,
        material: Arc::clone(&mat_right),
    });

    let mut cam = Camera::default();
    cam.aspect_ratio = 16. / 9.;
    cam.img_width = 400; // 2560
    cam.samples_per_pixel = 200; // 1000
    cam.max_bounces_per_ray = 50; // 100
    cam.vertical_fov = 35.;
    cam.look_from = Point3::from_floats(-2., 2., 0.);
    cam.look_at = Point3::from_floats(0., 0., -1.);
    cam.vup = Point3::from_floats(0., 1., 0.);

    cam.render(Arc::new(world));
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
