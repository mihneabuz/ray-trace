use std::io::{Result, Write};

use crate::{interval::Interval, vec::Vec3};

pub type Color = Vec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0. {
        linear_component.sqrt()
    } else {
        0.
    }
}

impl Color {
    pub fn write(&self, mut writer: impl Write) -> Result<()> {
        let r = linear_to_gamma(self.x());
        let g = linear_to_gamma(self.y());
        let b = linear_to_gamma(self.z());

        let intensity = Interval::new(0., 0.9999);
        let ir = (intensity.clamp(r) * 255.999) as i32;
        let ig = (intensity.clamp(g) * 255.999) as i32;
        let ib = (intensity.clamp(b) * 255.999) as i32;

        write!(writer, "{ir} {ig} {ib}\n")
    }
}
