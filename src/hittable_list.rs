use crate::{hittable::HitRecord, hittable::Hittable, interval::Interval, ray::Ray};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    pub fn new(objects: Vec<Box<dyn Hittable + Send + Sync>>) -> Self {
        Self { objects }
    }

    // pub fn push(&mut self, hittable: Box<dyn Hittable>) {
    //     self.objects.push(hittable);
    // }

    pub fn hit(&self, ray: &Ray, rayt: Interval, hit_record: &mut HitRecord) -> bool {
        // temp_rec needs to be initialized in order to get passed into object.hit
        let mut temp_rec: HitRecord = HitRecord::blank();
        let mut hit_anything: bool = false;
        let mut closest_so_far = rayt.max;

        for object in &self.objects {
            if object.hit(ray, Interval::new(rayt.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;

                // update hit record like this because it doesn't work when you update it all at once
                hit_record.point = temp_rec.point.clone();
                hit_record.normal = temp_rec.normal.clone();
                hit_record.t = temp_rec.t;
                hit_record.front_face = temp_rec.front_face;
                hit_record.material = temp_rec.material.clone();
            }
        }

        hit_anything
    }
}
