use std::rc::Rc;

use crate::{
    interval::Interval,
    material::Material,
    vec::{Point3, Vec3},
};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}

pub struct World {
    objects: Vec<Box<dyn Hitable>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add<T>(&mut self, object: T)
    where
        T: Hitable + 'static,
    {
        self.objects.push(Box::new(object));
    }
}

impl Hitable for World {
    fn hit(&self, ray: &Ray, t: Interval) -> Option<Hit> {
        let mut hit = None;
        let mut closest = t.max;

        for obj in &self.objects {
            if let Some(new) = obj.hit(ray, Interval::new(t.min, closest)) {
                closest = new.t;
                hit = Some(new);
            }
        }

        hit
    }
}

pub struct Hit {
    pub t: f64,
    pub point: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t: Interval) -> Option<Hit>;
}

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius: radius.max(0.),
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t: Interval) -> Option<Hit> {
        let oc = self.center - *ray.origin();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0. {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if !t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !t.surrounds(root) {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - self.center) / self.radius;
        let front_face = ray.direction().dot(&outward_normal) < 0.;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Some(Hit {
            t: root,
            point,
            normal,
            front_face,
            material: Rc::clone(&self.material),
        })
    }
}
