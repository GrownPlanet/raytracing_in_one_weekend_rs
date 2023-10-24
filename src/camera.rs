use rand::Rng;

use crate::hittable::HitRecord;
use crate::interval::Interval;
use crate::ray::Ray;

use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::point3::Point3;

pub struct Camera {
    image_width: i32,
    image_height: i32,
    center: Point3,
    pixel00: Point3,
    pixel_delta_u: Point3,
    pixel_delta_v: Point3,

    sampels_per_pixel: i32,
    max_depth: i32,
}

impl Camera {
    pub fn init(
        aspect_ratio: f64,
        image_width: i32,
        sampels_per_pixel: i32,
        max_depth: i32,
    ) -> Self {
        // calculate image height
        let mut image_height = (image_width as f64 / aspect_ratio) as i32;
        if image_height < 1 {
            image_height = 1;
        }

        // camera
        let focal_length = 1.;

        let fov = 90.;
        let theta = degrees_to_radians(fov);
        let h = (theta / 2.).tan();

        let viewport_height = 2. * h * focal_length;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let center = Point3::new(0., 0., 0.3);

        // calculate vectors accros the viewport edges
        let viewport_u = Point3::new(viewport_width, 0., 0.);
        let viewport_v = Point3::new(0., -viewport_height, 0.);

        // calculate delta vectors
        let pixel_delta_u = viewport_u.clone() / image_width as f64;
        let pixel_delta_v = viewport_v.clone() / image_height as f64;

        // calculate the location of the upper left pixel
        let viewport_upper_left = center.clone()
            - Point3::new(0., 0., focal_length)
            - viewport_u.clone() / 2.
            - viewport_v.clone() / 2.;

        let pixel00 =
            viewport_upper_left.clone() + (pixel_delta_u.clone() + pixel_delta_v.clone()) / 2.;

        Self {
            image_width,
            image_height,
            center,
            pixel00,
            pixel_delta_u,
            pixel_delta_v,
            sampels_per_pixel,
            max_depth,
        }
    }

    pub fn render_part(
        &self,
        world: &HittableList,
        part: i32,
        part_amount: i32,
    ) -> Vec<(i32, i32, Color)> {
        // println!("-------------- part {}: Starting --------------", part);

        let mut return_vec: Vec<(i32, i32, Color)> = vec![];

        let start_y =
            (part as f64 * (self.image_height as f64 / part_amount as f64)).round() as i32;
        let end_y =
            ((part as f64 + 1.) * (self.image_height as f64 / part_amount as f64)).round() as i32;

        for k in start_y..end_y {
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0., 0., 0.);

                for _ in 0..self.sampels_per_pixel {
                    let r = self.get_ray(i, k);
                    pixel_color = pixel_color + Self::ray_color(&r, &world, self.max_depth);
                }

                return_vec.push((i, k, pixel_color));
            }
        }
        // println!("-------------- part {}: Done ------------------", part);

        return_vec
    }

    fn ray_color(ray: &Ray, world: &HittableList, depth: i32) -> Color {
        if depth <= 0 {
            return Color::new(0., 0., 0.);
        }

        let mut record = HitRecord::blank();

        if world.hit(ray, Interval::new(0.001, f64::MAX), &mut record) {
            let mut scatterd = Ray::default();
            let mut attentuation = Color::default();

            if record
                .material
                .scatter(ray, &record, &mut attentuation, &mut scatterd)
            {
                return attentuation * Self::ray_color(&scatterd, &world, depth - 1);
            }
            return Color::default();
        }

        // change the vector to a value between `-1` and `1`
        let unit_direcion = ray.dir.unit_vector();
        // map teh value from `-1` to `1` to a value between `0` and `1`
        let a = 0.5 * (unit_direcion.y + 1.);

        // if a is 0, draw white, if 1 draw blue and inbetween
        Color::new(1., 1., 1.) * (1. - a) + Color::new(0.5, 0.7, 1.0) * a
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center = self.pixel00.clone()
            + (self.pixel_delta_u.clone() * i as f64)
            + (self.pixel_delta_v.clone() * j as f64);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center.clone();
        let ray_dir = pixel_sample - ray_origin.clone();

        return Ray::new(ray_origin, ray_dir);
    }

    fn pixel_sample_square(&self) -> Point3 {
        let mut rng = rand::thread_rng();

        let mut px: f64 = rng.gen();
        px *= -0.5;
        let mut py: f64 = rng.gen();
        py *= -0.5;

        return (self.pixel_delta_u.clone() * px) + (self.pixel_delta_v.clone() * py);
    }
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * (std::f64::consts::PI / 180.0)
}
