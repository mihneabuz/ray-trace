use std::{ops::Neg, rc::Rc};

use crate::{
    color::Color,
    ray::{Hit, Ray},
    util,
};

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter>;
}

pub fn lambertian(albedo: Color) -> Rc<dyn Material> {
    Rc::new(Labertian { albedo })
}

pub fn metal(albedo: Color, fuzz: f64) -> Rc<dyn Material> {
    Rc::new(Metal { albedo, fuzz })
}

pub fn dielectric(refraction_index: f64) -> Rc<dyn Material> {
    Rc::new(Dielectric { refraction_index })
}

struct Labertian {
    albedo: Color,
}

impl Material for Labertian {
    fn scatter(&self, _: &Ray, hit: &Hit) -> Option<Scatter> {
        let mut direction = hit.normal + util::rand_vec_unit();

        if direction.near_zero() {
            direction = hit.normal;
        }

        return Some(Scatter {
            ray: Ray::new(hit.point, direction),
            attenuation: self.albedo,
        });
    }
}

struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let mut reflected = util::reflect(ray.direction(), &hit.normal);
        reflected = reflected.unit() + (util::rand_vec_unit() * self.fuzz);

        if reflected.dot(&hit.normal) <= 0. {
            return None;
        }

        return Some(Scatter {
            ray: Ray::new(hit.point, reflected),
            attenuation: self.albedo,
        });
    }
}

struct Dielectric {
    refraction_index: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let ri = if hit.front_face {
            1. / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction().unit();
        let cos_theta = unit_direction.neg().dot(&hit.normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.;
        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > util::rand() {
            util::reflect(&unit_direction, &hit.normal)
        } else {
            util::refract(&unit_direction, &hit.normal, ri)
        };

        return Some(Scatter {
            ray: Ray::new(hit.point, direction),
            attenuation: Color::one(),
        });
    }
}

impl Dielectric {
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1. - refraction_index) / (1. + refraction_index);
        r0 = r0 * r0;
        r0 + (1. - r0) * (1. - cosine).powi(5)
    }
}
