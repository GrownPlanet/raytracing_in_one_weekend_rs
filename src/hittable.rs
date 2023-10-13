use crate::{interval::Interval, point3::Point3, ray::Ray};

#[derive(Default)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Point3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Point3) {
        self.front_face = Point3::dot(&ray.dir, outward_normal) < 0.;

        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            outward_normal.clone() * -1.
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, rayt: Interval, hit_record: &mut HitRecord) -> bool;
}
