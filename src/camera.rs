use crate::{
    color::Color,
    image::Image,
    interval::Interval,
    ray::{Hitable, Ray, World},
    util,
    vec::{Point3, Vec3},
};

struct Viewport {
    center: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_start: Point3,
}

impl Viewport {
    fn new(width: u32, height: u32, center: Vec3) -> Self {
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

        let pixel_start = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Self {
            center,
            pixel_delta_u,
            pixel_delta_v,
            pixel_start,
        }
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = util::sample_square();
        let pixel_sample = self.pixel_start
            + self.pixel_delta_u * (i as f64 + offset.x())
            + self.pixel_delta_v * (j as f64 + offset.y());

        Ray::new(self.center, pixel_sample - self.center)
    }
}

pub struct Camera {
    width: u32,
    height: u32,
    viewport: Viewport,
    samples_per_pixel: u16,
    samples_scale: f64,
    max_depth: u32,
}

#[derive(Debug, Clone)]
pub enum Window {
    Fixed { width: u32, height: u32 },
    Ratio { width: u32, aspect: f64 },
}

impl Window {
    pub fn new(width: u32, height: u32) -> Self {
        Self::Fixed { width, height }
    }

    pub fn with_aspect_ratio(width: u32, aspect_ratio: f64) -> Self {
        Self::Ratio {
            width,
            aspect: aspect_ratio,
        }
    }

    fn size(&self) -> (u32, u32) {
        match *self {
            Window::Fixed { width, height } => (width, height),
            Window::Ratio { width, aspect } => {
                let height = ((width as f64 / aspect) as u32).max(1);
                (width, height)
            }
        }
    }
}

pub struct CameraBuilder {
    window: Window,
}

impl CameraBuilder {
    pub fn new() -> Self {
        Self {
            window: Window::new(400, 400),
        }
    }

    pub fn window(mut self, window: Window) -> Self {
        self.window = window;
        self
    }

    pub fn build(self) -> Camera {
        let (width, height) = self.window.size();
        let viewport = Viewport::new(width, height, Point3::zero());
        Camera {
            width,
            height,
            viewport,
            samples_per_pixel: 16,
            samples_scale: 1. / 16.,
            max_depth: 10,
        }
    }
}

impl Camera {
    pub fn builder() -> CameraBuilder {
        CameraBuilder::new()
    }

    pub fn render(&self, world: &World) -> Image {
        let mut data = Vec::with_capacity((self.height * self.width) as usize);

        for j in 0..self.height {
            for i in 0..self.width {
                let color = (0..self.samples_per_pixel).fold(Color::zero(), |color, _| {
                    color + Self::ray_color(&self.viewport.get_ray(i, j), self.max_depth, world)
                });

                data.push(color * self.samples_scale);
            }
        }

        Image::new(self.width, self.height, data).unwrap()
    }

    fn ray_color(ray: &Ray, depth: u32, world: &World) -> Color {
        if depth == 0 {
            return Color::zero();
        }

        if let Some(hit) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            if let Some(scatter) = hit.material.scatter(ray, &hit) {
                return scatter.attenuation * Self::ray_color(&scatter.ray, depth - 1, world);
            };
            let direction = hit.normal + util::rand_vec_unit();
            return Self::ray_color(&Ray::new(hit.point, direction), depth - 1, world) * 0.5;
        }

        let unit = ray.direction().unit();
        let a = 0.5 * (unit.y() + 1.);
        Color::one() * (1. - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}
