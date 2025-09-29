use std::f64::consts::PI;

use crate::vec::Vec3;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.
}

pub fn rand() -> f64 {
    rand::random_range(0.0..1.0)
}

pub fn rand_range(min: f64, max: f64) -> f64 {
    rand::random_range(min..max)
}

pub fn sample_square() -> Vec3 {
    Vec3::new(rand() - 0.5, rand() - 0.5, 0.)
}

pub fn rand_vec() -> Vec3 {
    Vec3::new(rand(), rand(), rand())
}

pub fn rand_vec_range(min: f64, max: f64) -> Vec3 {
    Vec3::new(
        rand_range(min, max),
        rand_range(min, max),
        rand_range(min, max),
    )
}

pub fn rand_vec_unit() -> Vec3 {
    loop {
        let v = rand_vec_range(-1., 1.);
        let len = v.length_squared();
        if 1e-160 < len && len <= 1. {
            return v / len.sqrt();
        }
    }
}

pub fn rand_vec_on_hemisphere(normal: &Vec3) -> Vec3 {
    let v = rand_vec_unit();
    if v.dot(normal) > 0. { v } else { -v }
}

pub fn reflect(v: &Vec3, normal: &Vec3) -> Vec3 {
    *v - *normal * v.dot(normal) * 2.
}

pub fn refract(v: &Vec3, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (-*v).dot(normal).min(1.);
    let r_out_perp = (*v + *normal * cos_theta) * etai_over_etat;
    let r_out_parallel = *normal * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
    r_out_perp + r_out_parallel
}
