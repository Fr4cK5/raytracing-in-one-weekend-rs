use std::{
    fs,
    sync::{Arc, Mutex},
    time::Instant,
};

use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{
    interval::Interval,
    ray::Ray,
    utils,
    vec3::{Color, Point3, Vec3},
    world::{AnyHit, World},
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub img_width: i32,
    pub samples_per_pixel: i32,
    pub max_bounces_per_ray: i32,
    pub vertical_fov: f64,
    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: Point3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    pixel_samples_scale: f64,
    img_height: i32,
    center: Point3,
    first_pixel: Point3,
    pixel_delta_x: Vec3,
    pixel_delta_y: Vec3,
    x: Vec3,
    y: Vec3,
    z: Vec3,
    defocus_disk_x: Vec3,
    defocus_disk_y: Vec3,
}

impl Camera {
    pub fn render(&mut self, world: Arc<World>) {
        self.init();

        let headers = format!("P3\n{} {}\n255\n", self.img_width, self.img_height);
        let buf = Arc::new(Mutex::new(Vec::<String>::with_capacity(
            self.img_width as usize * self.img_height as usize,
        )));

        let mut buf_lock = buf.lock().unwrap();
        buf_lock.resize_with(
            self.img_width as usize * self.img_height as usize,
            Default::default,
        );
        drop(buf_lock);

        let start = Instant::now();
        (0..self.img_height).par_bridge().for_each(|y| {
            for x in 0..self.img_width {
                let mut col = Color::default();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(x, y);
                    col += self.ray_color(&r, &world, 1);
                }

                utils::write_color(
                    Arc::clone(&buf),
                    &col.mul(self.pixel_samples_scale),
                    x as usize,
                    y as usize,
                    self.img_width as usize,
                );
            }
        });

        println!("\nRT took: {:?}", start.elapsed());

        let buf_lock = buf.lock().unwrap();

        let out = headers
            .chars()
            .chain(buf_lock.join("\n").chars())
            .collect::<String>();

        drop(buf_lock);

        fs::write("test-img.ppm", out.as_bytes()).expect("Unable to write to file");
    }

    fn init(&mut self) {
        self.img_height = ((self.img_width as f64 / self.aspect_ratio) as i32).max(1);
        self.pixel_samples_scale = 1. / self.samples_per_pixel as f64;

        self.center = self.look_from;

        // Viewoprt Dimensions
        let theta = self.vertical_fov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h * self.focus_dist;
        let viewport_width = viewport_height * (self.img_width as f64 / self.img_height as f64);

        // x, y, z unit basis vectors for camera coordinate frame
        self.z = (self.look_from - self.look_at).norm();
        self.x = self.vup.cross(&self.z).norm();
        self.y = self.z.cross(&self.x);

        // Viewport Vectors
        let viewport_x = self.x.mul(viewport_width);
        let viewport_y = self.y.inv().mul(viewport_height);

        // Viewport Delta Vectors
        self.pixel_delta_x = viewport_x.div(self.img_width as f64);
        self.pixel_delta_y = viewport_y.div(self.img_height as f64);

        let viewport_upper_left =
            self.center - self.z.mul(self.focus_dist) - viewport_x.div(2.) - viewport_y.div(2.);

        self.first_pixel = viewport_upper_left + (self.pixel_delta_x + self.pixel_delta_y).mul(0.5);

        // Camera Defocus of upper left pixel
        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.).to_radians().tan();
        self.defocus_disk_x = self.x.mul(defocus_radius);
        self.defocus_disk_y = self.y.mul(defocus_radius);
    }

    fn ray_color(&self, r: &Ray, world: &World, depth: i32) -> Color {
        if depth >= self.max_bounces_per_ray {
            return Color::default();
        }

        if let Some(hit) = world.any_hit(r, Interval::new(0.001, f64::INFINITY)) {
            if let Some(mat) = &hit.material {
                if let Some(scat) = mat.scatter(r, &hit) {
                    return scat.attenuation * self.ray_color(&scat.ray, world, depth + 1);
                }
            }
            return Color::from_floats(0., 0., 0.);
        }

        let unit_dir = r.direction.norm();
        let a = (unit_dir.y() + 1.) * 0.5;
        return Color::from_floats(1., 1., 1.).mul(1. - a)
            + Color::from_floats(0.5, 0.7, 1.).mul(a);
    }

    fn get_ray(&self, x: i32, y: i32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.first_pixel
            + (self.pixel_delta_x.mul(x as f64 + offset.x()))
            + (self.pixel_delta_y.mul(y as f64 + offset.y()));

        let ray_origin = if self.defocus_angle <= 0. {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_dir = pixel_sample - ray_origin;

        return Ray::new(ray_origin, ray_dir);
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        return self.center + (self.defocus_disk_x.mul(p.0)) + (self.defocus_disk_y.mul(p.1));
    }

    fn sample_square() -> Vec3 {
        return Vec3(utils::rand_float() - 0.5, utils::rand_float() - 0.5, 0.);
    }
}

impl Default for Camera {
    fn default() -> Self {
        return Self {
            aspect_ratio: 1.,
            img_width: 100,
            samples_per_pixel: 10,
            max_bounces_per_ray: 10,
            look_from: Vec3::from_floats(0., 0., 0.),
            look_at: Vec3::from_floats(0., 0., -1.),
            vup: Vec3::from_floats(0., 1., 0.),
            vertical_fov: 90.,
            pixel_samples_scale: 1.,
            defocus_angle: 0.,
            focus_dist: 10.,
            img_height: 0,
            center: Point3::default(),
            first_pixel: Point3::default(),
            pixel_delta_x: Vec3::default(),
            pixel_delta_y: Vec3::default(),
            x: Vec3::default(),
            y: Vec3::default(),
            z: Vec3::default(),
            defocus_disk_x: Vec3::default(),
            defocus_disk_y: Vec3::default(),
        };
    }
}
