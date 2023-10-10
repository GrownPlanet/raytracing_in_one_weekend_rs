use crate::{
    hittable::{HitRecord, Hittable},
    point3::Point3,
    ray::Ray,
};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, rayt_min: f64, rayt_max: f64, hit_record: &mut HitRecord) -> bool {
        let oc = ray.orig.clone() - self.center.clone();

        let a = ray.dir.len_squared();
        let half_b = Point3::dot(&oc, &ray.dir);
        let c = oc.len_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0. {
            return false;
        }

        let discriminant_sqrt = discriminant.sqrt();

        // find the nearest root in acceptable range

        let mut root = (-half_b - discriminant_sqrt) / a;
        if root <= rayt_min || rayt_max <= root {
            root = (-half_b + discriminant_sqrt) / a;
            if root <= rayt_min || rayt_max <= root {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.point = ray.at(root);
        let outward_normal = (hit_record.point.clone() - self.center.clone()) / self.radius;
        hit_record.set_face_normal(ray, &outward_normal);
        hit_record.normal = (hit_record.point.clone() - self.center.clone()) / self.radius;

        true
    }
}
