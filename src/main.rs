use std::{
    fs::{self, File},
    path::Path,
};

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod point3;
mod ray;
mod sphere;

use camera::Camera;
use hittable_list::HittableList;
use point3::Point3;
use sphere::Sphere;

fn main() {
    // open file to put image in
    let path = "image.ppm";
    if Path::new(path).exists() {
        fs::remove_file(path).unwrap();
    }

    let file = File::create(path).unwrap();

    // world
    let world = HittableList::new(vec![
        Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)),
        Box::new(Sphere::new(Point3::new(0., -100.5, 0.), 100.)),
    ]);

    let mut camera = Camera::init(16. / 9., 400, file);

    camera.render(&world);
}
