pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    const EMPTY: Self = Self::new(f64::INFINITY, -f64::INFINITY);
    const UNIVERSE: Self = Self::new(-f64::INFINITY, f64::INFINITY);

    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub const fn size(&self) -> f64 {
        self.max - self.min
    }

    pub const fn contains(&self, x: f64) -> bool {
        self.min <= x && self.max >= x
    }

    pub const fn surrounds(&self, x: f64) -> bool {
        self.min < x && self.max > x
    }

    pub const fn clamp(&self, x: f64) -> f64 {
        x.max(self.min).min(self.max)
    }
}
