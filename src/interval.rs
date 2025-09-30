pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub const fn surrounds(&self, x: f64) -> bool {
        self.min < x && self.max > x
    }

    pub const fn clamp(&self, x: f64) -> f64 {
        x.max(self.min).min(self.max)
    }
}
