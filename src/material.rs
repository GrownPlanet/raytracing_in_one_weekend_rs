use crate::{color::Color, hittable::HitRecord, point3::Point3, ray::Ray};

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attentuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Clone, Default)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        record: &HitRecord,
        attentuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut direction = record.normal.clone() + Point3::random_unit_vector();

        if direction.near_zero() {
            direction = record.normal.clone();
        }

        *scattered = Ray::new(record.point.clone(), direction);

        *attentuation = self.albedo.clone();

        true
    }
}

#[derive(Clone, Default)]
pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attentuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Point3::reflect(&ray_in.dir.unit_vector(), &record.normal);

        // println!("{} {} {}", self.albedo.r, self.albedo.g, self.albedo.b);

        *scattered = Ray::new(record.point.clone(), reflected);
        *attentuation = self.albedo.clone();

        true
    }
}
