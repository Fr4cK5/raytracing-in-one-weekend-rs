// https://raytracing.github.io/books/RayTracingInOneWeekend.html

#![allow(clippy::needless_return)]

use core::f64;
use std::{fs, time::Instant};

use ray::Ray;
use vec3::{Color, Point3, Vec3};
use world::{World, AnyHit};

mod color_utils;
mod ray;
mod vec3;
mod sphere;
mod hit;
mod world;

fn main() {

    // Image
    let aspect_ratio = 16. / 9.;
    let img_width = 400;
    let img_height = ((img_width as f64 / aspect_ratio) as i32).max(1);

    let mut world = World::new();
    world.push(sphere::Sphere { center: Vec3::from_floats(0., 0., -1.), radius: 0.5 });
    world.push(sphere::Sphere { center: Vec3::from_floats(0., -100.5, -1.), radius: 100. });

    // Camera & Viewport Scale
    let focal_length = 1.;
    let camera_center = Point3::from_floats(0., 0., 0.);
    let viewport_height = 2.;
    let viewport_width = viewport_height * (img_width as f64 / img_height as f64);

    // Viewport Vectors
    let viewport_x = Vec3::from_floats(viewport_width, 0., 0.);
    let viewport_y = Vec3::from_floats(0., -viewport_height, 0.);

    // Viewport Delta Vectors
    let pixel_delta_x = viewport_x.div(img_width as f64);
    let pixel_delta_y = viewport_y.div(img_height as f64);

    let viewport_upper_left =
        camera_center - Vec3(0., 0., focal_length) - viewport_x.div(2.) - viewport_y.div(2.);

    let first_pixel = viewport_upper_left + (pixel_delta_x + pixel_delta_y).mul(0.5);

    let headers = format!("P3\n{img_width} {img_height}\n255\n");
    let mut buf = Vec::<String>::new();

    let start = Instant::now();
    for y in 0..img_height {
        for x in 0..img_width {
            let pixel_center =
                first_pixel + (pixel_delta_x.mul(x as f64) + pixel_delta_y.mul(y as f64));
            let ray_dir = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_dir);
            let col = ray_color(&r, &world);
            color_utils::write_color(&mut buf, &col);
        }
    }

    println!("RT took: {:?}", start.elapsed());

    let out = headers
        .chars()
        .chain(buf.join("\n").chars())
        .collect::<String>();

    fs::write("test-img.ppm", out.as_bytes()).expect("Unable to write to file.");
}

pub fn ray_color(r: &Ray, world: &World) -> Color {
    if let Some(hit) = world.any_hit(r, 0., f64::INFINITY) {
        return (hit.normal + Color::from_floats(1., 1., 1.)).mul(0.5);
    }

    let unit_dir = r.direction.norm();
    let a = (unit_dir.y() + 1.) * 0.5;
    return Color::from_floats(1., 1., 1.).mul(1. - a) + Color::from_floats(0.5, 0.7, 1.).mul(a);
}

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
