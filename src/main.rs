// https://raytracing.github.io/books/RayTracingInOneWeekend.html

#![allow(clippy::needless_return)]

use core::f64;
use std::sync::Arc;

use camera::Camera;
use material::{dialectric::Dialectric, lambertian::Lambertian, metal::Metal, Material};
use ray::Ray;
use sphere::Sphere;
use vec3::{Color, Point3, Vec3};
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
        albedo: Vec3(0.6, 0.5, 0.7),
    });

    let mut world = World::new();
    world.push(Sphere::new(
        Point3::from_floats(0., -1000., 0.),
        1000.,
        Arc::clone(&mat_ground),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let material_mode = utils::rand_float();
            let center = Point3::from_floats(
                a as f64 + 0.9 * utils::rand_float(),
                0.2,
                b as f64 + 0.9 * utils::rand_float(),
            );

            if (center - Point3::from_floats(4., 0.2, 0.)).len() > 0.9 {
                let sphere_material: Arc<dyn Material>;

                if material_mode < 0.7 {
                    // Diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian { albedo });
                    world.push(Sphere::new(center, 0.2, Arc::clone(&sphere_material)));
                } else if material_mode < 0.9 {
                    // Metal
                    let albedo = Color::random_range(0.5, 1.);
                    let fuzz = utils::rand_float_range(0., 0.5);
                    sphere_material = Arc::new(Metal { albedo, fuzz });
                    world.push(Sphere::new(center, 0.2, Arc::clone(&sphere_material)));
                } else {
                    // Dialectric/Glass
                    sphere_material = Arc::new(Dialectric {
                        refrecation_index: 1.5,
                    });
                    world.push(Sphere::new(center, 0.2, Arc::clone(&sphere_material)));
                }
            }
        }
    }

    let mat: Arc<dyn Material> = Arc::new(Dialectric {
        refrecation_index: 1.5,
    });
    world.push(Sphere::new(Point3::from_floats(0., 1., 0.), 1., mat));

    let mat: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: Color::from_floats(0.4, 0.2, 0.1),
    });
    world.push(Sphere::new(Point3::from_floats(-4., 1., 0.), 1., mat));

    let mat: Arc<dyn Material> = Arc::new(Metal {
        albedo: Color::from_floats(0.7, 0.6, 0.5),
        fuzz: 0.,
    });
    world.push(Sphere::new(Point3::from_floats(4., 1., 0.), 1., mat));

    let mut cam = Camera::default();
    cam.aspect_ratio = 16. / 9.;
    cam.img_width = 400; // 2560
    cam.samples_per_pixel = 200; // 1000
    cam.max_bounces_per_ray = 50; // 100

    cam.vertical_fov = 30.;
    cam.look_from = Point3::from_floats(13., 2., 3.);
    cam.look_at = Point3::from_floats(0., 0., 0.);
    cam.vup = Point3::from_floats(0., 1., 0.);

    cam.defocus_angle = 0.6; // 10, set to 0 to remove the Defocus Blur (DoF)
    cam.focus_dist = 10.; // 3.4

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
