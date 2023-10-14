pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    // pub fn contains(&self, x: f64) -> bool {
    //     self.min <= x && self.max >= x
    // }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && self.max > x
    }
}
