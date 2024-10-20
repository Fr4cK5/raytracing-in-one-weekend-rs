use std::{fs, time::Instant};

use crate::{
    color_utils, interval::Interval, ray::Ray, vec3::{Color, Point3, Vec3}, world::{AnyHit, World}
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub img_width: i32,
    img_height: i32,
    center: Point3,
    first_pixel: Point3,
    pixel_delta_x: Vec3,
    pixel_delta_y: Vec3,
}

impl Camera {
    pub fn render(&mut self, world: &World) {
        self.init();

        let headers = format!("P3\n{} {}\n255\n", self.img_width, self.img_height);
        let mut buf = Vec::<String>::new();

        let start = Instant::now();
        for y in 0..self.img_height {
            for x in 0..self.img_width {
                let pixel_center =
                    self.first_pixel + (self.pixel_delta_x.mul(x as f64) + self.pixel_delta_y.mul(y as f64));
                let ray_dir = pixel_center - self.center;
                let r = Ray::new(self.center, ray_dir);
                let col = Self::ray_color(&r, &world);
                color_utils::write_color(&mut buf, &col);
            }
        }

        println!("RT took: {:?}", start.elapsed());

        let out = headers
            .chars()
            .chain(buf.join("\n").chars())
            .collect::<String>();

        fs::write("test-img.ppm", out.as_bytes()).expect("Unable to write to file");
    }

    fn init(&mut self) {
        self.img_height = ((self.img_width as f64 / self.aspect_ratio) as i32).max(1);

        // Viewoprt Dimensions
        let focal_length = 1.;
        let viewport_height = 2.;
        let viewport_width = viewport_height * (self.img_width as f64 / self.img_height as f64);

        // Viewport Vectors
        let viewport_x = Vec3::from_floats(viewport_width, 0., 0.);
        let viewport_y = Vec3::from_floats(0., -viewport_height, 0.);

        // Viewport Delta Vectors
        self.pixel_delta_x = viewport_x.div(self.img_width as f64);
        self.pixel_delta_y = viewport_y.div(self.img_height as f64);

        let viewport_upper_left =
            self.center - Vec3(0., 0., focal_length) - viewport_x.div(2.) - viewport_y.div(2.);

        self.first_pixel = viewport_upper_left + (self.pixel_delta_x + self.pixel_delta_y).mul(0.5);
    }

    fn ray_color(r: &Ray, world: &World) -> Color {
        if let Some(hit) = world.any_hit(r, Interval::new(0., f64::INFINITY)) {
            return (hit.normal + Color::from_floats(1., 1., 1.)).mul(0.5);
        }

        let unit_dir = r.direction.norm();
        let a = (unit_dir.y() + 1.) * 0.5;
        return Color::from_floats(1., 1., 1.).mul(1. - a)
            + Color::from_floats(0.5, 0.7, 1.).mul(a);
    }
}

impl Default for Camera {
    fn default() -> Self {
        return Self {
            aspect_ratio: 1.,
            img_width: 100,
            img_height: 0,
            center: Point3::default(),
            first_pixel: Point3::default(),
            pixel_delta_x: Vec3::default(),
            pixel_delta_y: Vec3::default(),
        };
    }
}
