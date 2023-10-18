use crate::point3::Point3;

#[derive(Default)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Point3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Point3) -> Self {
        Self { orig, dir }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig.clone() + self.dir.clone() * t
    }
}
