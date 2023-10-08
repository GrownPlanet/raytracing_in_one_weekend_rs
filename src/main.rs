use std::{
    fs::{self, File},
    io::prelude::*,
    path::Path,
};

pub mod color;
pub mod point3;
pub mod ray;

use ray::Ray;

use crate::color::Color;
use crate::point3::Point3;

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
        println!("Scanlines remaining: {}", (image_height - j));
        for i in 0..image_width {
            let pixel_center = pixel00_loc.clone()
                + (pixel_delta_u.clone() * i as f64)
                + (pixel_delta_v.clone() * j as f64);

            let ray_direction = pixel_center - camera_center.clone();

            let ray = Ray::new(camera_center.clone(), ray_direction);

            let pixel_color = ray_color(&ray);

            write!(file, "{}", pixel_color.to_string()).unwrap();
        }
    }
    println!("-------------- Done --------------")
}

fn ray_color(ray: &Ray) -> Color {
    Color::new(0., 0., 0.)
}
