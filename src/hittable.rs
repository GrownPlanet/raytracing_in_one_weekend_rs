use crate::{point3::Point3, ray::Ray};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Point3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, rayt_min: f64, rayt_max: f64, hit_record: &mut HitRecord) -> bool;
}
