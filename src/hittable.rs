use std::rc::Rc;

use crate::{
    interval::Interval,
    material::{Lambertian, Material},
    point3::Point3,
    ray::Ray,
};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Point3,
    pub t: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
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

    pub fn blank() -> Self {
        Self {
            point: Point3::default(),
            normal: Point3::default(),
            t: 0.,
            front_face: false,
            material: Rc::new(Lambertian::default()),
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, rayt: Interval, hit_record: &mut HitRecord) -> bool;
}
