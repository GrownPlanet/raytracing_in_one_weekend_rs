use std::{
    fs::{self, File},
    io::prelude::*,
    path::Path,
    rc::Rc,
    time::Instant,
};

use rayon::prelude::*;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod point3;
mod ray;
mod sphere;

use camera::Camera;
use color::Color;
use hittable_list::HittableList;
use point3::Point3;
use sphere::Sphere;

use crate::material::{Lambertian, Metal};

fn main() {
    // open file to put image in
    let path = "image.ppm";
    if Path::new(path).exists() {
        fs::remove_file(path).unwrap();
    }

    let mut file = File::create(path).unwrap();

    // world
    let world = HittableList::new(vec![
        Box::new(Sphere::new(
            Point3::new(0., 0., -1.),
            0.5,
            Rc::new(Lambertian::new(Color::new(0.3, 0.1, 0.7))),
        )),
        Box::new(Sphere::new(
            Point3::new(1., 0., -1.),
            0.5,
            Rc::new(Metal::new(Color::new(0.7, 0.2, 0.4), 0.5)),
        )),
        Box::new(Sphere::new(
            Point3::new(-1., 0., -1.),
            0.5,
            Rc::new(Metal::new(Color::new(0.5, 0.5, 0.5), 0.2)),
        )),
        Box::new(Sphere::new(
            Point3::new(0., -100.5, -1.),
            100.,
            Rc::new(Lambertian::new(Color::new(0.3, 0.6, 0.1))),
        )),
    ]);

    let aspect_ratio = 16. / 9.;
    let image_width = 400;
    let image_height = image_width as f64 / aspect_ratio;

    let camera = Camera::init(aspect_ratio, image_width, 100, 50);

    let part_amount = 10;

    let start = Instant::now();

    let result: Vec<String> = (0..part_amount)
        .into_par_iter()
        .map(|i| {
            // f
            camera.render_part(&world, i, part_amount)
        })
        .collect();

    let duration = start.elapsed();
    println!("time to render image: {:?}", duration);

    let result_string = result.join("");

    write!(file, "P3\n{} {}\n255\n", image_width, image_height as i32).unwrap();
    write!(file, "{}", result_string).unwrap();
}
