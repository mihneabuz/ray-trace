use std::io::Result;

use crate::{
    color::Color,
    image::Image,
    interval::Interval,
    ray::{Hitable, Ray, World},
    util,
    vec::{Point3, Vec3},
};

pub struct Camera {
    width: u32,
    height: u32,
    center: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Point3,
    samples_per_pixel: u16,
    samples_scale: f64,
    max_depth: u32,
    vfov: f64,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        let center = Point3::zero();

        let vfov = 90.;
        let focal_length = 1.;
        let theta = util::degrees_to_radians(vfov);
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h * focal_length;
        let viewport_width = viewport_height * (width as f64 / height as f64);

        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        let pixel_delta_u = viewport_u / width as f64;
        let pixel_delta_v = viewport_v / height as f64;

        let viewport_upper_left =
            center - Vec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;

        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let samples_per_pixel = 160;
        let samples_scale = 1. / samples_per_pixel as f64;

        let max_depth = 10;

        Self {
            width,
            height,
            center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            samples_per_pixel,
            samples_scale,
            max_depth,
            vfov,
        }
    }

    pub fn render(&self, world: &World) -> Result<()> {
        Image::write("image.ppm", self.width, self.height, |i, j| {
            let color = (0..self.samples_per_pixel).fold(Color::zero(), |color, _| {
                color + self.ray_color(&self.get_ray(i, j), self.max_depth, &world)
            });

            color * self.samples_scale
        })
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = util::sample_square();
        let pixel_sample = self.pixel00_loc
            + self.pixel_delta_u * (i as f64 + offset.x())
            + self.pixel_delta_v * (j as f64 + offset.y());

        Ray::new(self.center, pixel_sample - self.center)
    }

    fn ray_color(&self, ray: &Ray, depth: u32, world: &World) -> Color {
        if depth == 0 {
            return Color::zero();
        }

        if let Some(hit) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            if let Some(scatter) = hit.material.scatter(ray, &hit) {
                return scatter.attenuation * self.ray_color(&scatter.ray, depth - 1, world);
            };
            let direction = hit.normal + util::rand_vec_unit();
            return self.ray_color(&Ray::new(hit.point, direction), depth - 1, world) * 0.5;
        }

        let unit = ray.direction().unit();
        let a = 0.5 * (unit.y() + 1.);
        Color::one() * (1. - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}
