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
        // let mut direction = record.normal.clone();

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
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let mut fuzz = fuzz;
        if fuzz > 1. {
            fuzz = 1.;
        }
        Self { albedo, fuzz }
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

        *scattered = Ray::new(
            record.point.clone(),
            reflected + Point3::random_unit_vector() * self.fuzz,
        );
        *attentuation = self.albedo.clone();

        Point3::dot(&scattered.dir, &record.normal) > 0.
    }
}
