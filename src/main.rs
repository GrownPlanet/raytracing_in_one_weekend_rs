use std::{
    fs::{self, File},
    io::prelude::*,
    path::Path,
    rc::Rc,
    sync::Arc,
    time::Instant,
};

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
use rayon::prelude::*;
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

    let image_width = 300;
    write!(file, "P3\n{} {}\n255\n", image_width, image_width).unwrap();
    let part_a = 3;

    let camera = Camera::init(1., image_width, 100, 50);

    let start = Instant::now();

    let strings: Vec<String> = (0..part_a)
        .into_par_iter()
        .map(|i| camera.render_part(&world, i, part_a))
        .collect();

    let duration = start.elapsed();
    println!("time to render image: {:?}", duration);

    let mut yay = String::new();

    for s in strings.iter() {
        yay.push_str(&s);
    }

    // let yay = strings.join("");

    // write!(file, "{}", yay).unwrap();
}
