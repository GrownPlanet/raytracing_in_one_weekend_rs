use crate::point3::Point3;

pub struct Ray {
    orig: Point3,
    dir: Point3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Point3) -> Self {
        Self { orig, dir }
    }

    pub fn at(&mut self, t: f64) -> Point3 {
        self.orig.clone() + self.dir.clone() * t
    }
}
