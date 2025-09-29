mod camera;
mod color;
mod image;
mod interval;
mod material;
mod ray;
mod util;
mod vec;

use std::io::Result;

use crate::{
    camera::Camera,
    color::Color,
    material::{dielectric, lambertian, metal},
    ray::{Sphere, World},
    vec::Point3,
};

fn size(width: u32, aspect_ratio: f64) -> (u32, u32) {
    let height = ((width as f64 / aspect_ratio) as u32).max(1);
    (width, height)
}

fn main() -> Result<()> {
    let (width, height) = size(800, 16. / 9.);

    let material_ground = lambertian(Color::new(0.8, 0.8, 0.0));
    let material_center = lambertian(Color::new(0.1, 0.2, 0.5));
    let material_left = dielectric(1.5);
    let material_bubble = dielectric(1. / 1.5);
    let material_right = metal(Color::new(0.8, 0.6, 0.2), 1.0);

    let mut world = World::new();
    world.add(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground,
    ));
    world.add(Sphere::new(Point3::new(0., 0., -1.2), 0.5, material_center));
    world.add(Sphere::new(Point3::new(-1.0, 0., -1.), 0.5, material_left));
    world.add(Sphere::new(Point3::new(-1.0, 0., -1.), 0.4, material_bubble));
    world.add(Sphere::new(Point3::new(1.0, 0., -1.), 0.5, material_right));

    let camera = Camera::new(width, height);
    camera.render(&world)
}
