use std::{
    fs::{self, File},
    io::prelude::*,
    path::Path,
};

mod color;
mod hittable;
mod hittable_list;
mod interval;
mod point3;
mod ray;
mod sphere;

use hittable::HitRecord;
use interval::Interval;
use ray::Ray;

use crate::point3::Point3;
use crate::{color::Color, hittable_list::HittableList, sphere::Sphere};

fn main() {
    // open file to put image in
    let path = "image.ppm";
    if Path::new(path).exists() {
        fs::remove_file(path).unwrap();
    }

    let mut file = File::create(path).unwrap();

    // image dimensions
    let aspect_ratio = 16. / 9.;
    let image_width = 400;

    // calculate image height
    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    if image_height < 1 {
        image_height = 1;
    }

    // world
    let world = HittableList::new(vec![
        Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)),
        Box::new(Sphere::new(Point3::new(0., -100.5, 0.), 100.)),
    ]);

    // camera
    let focal_length = 1.;

    let viewport_height = 2.;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

    let camera_center = Point3::new(0., 0., 0.);

    // calculate vectors accros the viewport edges
    let viewport_u = Point3::new(viewport_width, 0., 0.);
    let viewport_v = Point3::new(0., -viewport_height, 0.);

    // calculate delta vectors
    let pixel_delta_u = viewport_u.clone() / image_width as f64;
    let pixel_delta_v = viewport_v.clone() / image_height as f64;

    // calculate the location of the upper left pixel
    let viewport_upper_left = camera_center.clone()
        - Point3::new(0., 0., focal_length)
        - viewport_u.clone() / 2.
        - viewport_v.clone() / 2.;

    let pixel00_loc =
        viewport_upper_left.clone() + (pixel_delta_u.clone() + pixel_delta_v.clone()) / 2.;

    // write basic ppm info to file
    write!(file, "P3\n{} {}\n255\n", image_width, image_height).unwrap();

    for j in 0..image_height {
        print!("Scanlines remaining: {} \r", (image_height - j));
        for i in 0..image_width {
            let pixel_center = pixel00_loc.clone()
                + (pixel_delta_u.clone() * i as f64)
                + (pixel_delta_v.clone() * j as f64);

            let ray_direction = pixel_center - camera_center.clone();

            let ray = Ray::new(camera_center.clone(), ray_direction);

            let pixel_color = ray_color(&ray, &world);

            write!(file, "{}", pixel_color.to_string()).unwrap();
        }
    }
    println!("-------------- Done --------------")
}

fn ray_color(ray: &Ray, world: &HittableList) -> Color {
    let mut record = HitRecord::default();

    if world.hit(ray, Interval::new(0., f64::MAX), &mut record) {
        return (Color::new(record.normal.x, record.normal.y, record.normal.z)
            + Color::new(1., 1., 1.))
            * 0.5;
    }

    // change the vector to a value between `-1` and `1`
    let unit_direcion = ray.dir.unit_vector();
    // map teh value from `-1` to `1` to a value between `0` and `1`
    let a = 0.5 * (unit_direcion.y + 1.);

    // if a is 0, draw white, if 1 draw blue and inbetween
    Color::new(1., 1., 1.) * (1. - a) + Color::new(0.5, 0.7, 1.0) * a
}
