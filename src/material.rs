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

#[derive(Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo: Color::new(255., 255., 255.),
        }
    }

    pub fn default() -> Self {
        Self {
            albedo: Color::new(1., 1., 1.),
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &Ray,
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
        // *attentuation = Color::new(0., 255., 255.);

        true
    }
}
