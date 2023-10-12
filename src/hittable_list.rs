use crate::{
    hittable::HitRecord,
    hittable::Hittable,
    point3::Point3,
    ray::{self, Ray},
};

struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn push(&mut self, hittable: Box<dyn Hittable>) {
        self.objects.push(hittable);
    }

    pub fn hit(&self, ray: &Ray, rayt_min: f64, rayt_max: f64, hit_record: &mut HitRecord) -> bool {
        // temp_rec needs to be initialized in order to get passed into object.hit
        let mut temp_rec: HitRecord = HitRecord {
            point: Point3::new(0., 0., 0.),
            normal: Point3::new(0., 0., 0.),
            t: 0.,
            front_face: false,
        };

        let mut hit_anything: bool = false;
        let mut closest_so_far = rayt_max;

        for object in &self.objects {
            if object.hit(ray, rayt_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;

                // update hit record like this because it doesn't work when you update it all at once
                // or i'm just stupid
                hit_record.point = temp_rec.point.clone();
                hit_record.normal = temp_rec.normal.clone();
                hit_record.t = temp_rec.t;
                hit_record.front_face = temp_rec.front_face;
            }
        }

        hit_anything
    }
}
