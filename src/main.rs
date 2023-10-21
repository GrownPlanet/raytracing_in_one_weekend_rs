use std::{
    fs::{self, File},
    io::prelude::*,
    path::Path,
    rc::Rc,
    string,
    sync::{Arc, Mutex},
    thread,
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
    let world = Arc::new(HittableList::new(vec![
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
    ]));

    let image_width = 800;
    let ih = (image_width as f64 / (16. / 9.)) as i32;
    write!(file, "P3\n{} {}\n255\n", 800, ih).unwrap();
    let part_a = 12;
    let strings: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![String::new(); 64]));

    let mut handles = vec![];

    let camera = Arc::new(Camera::init(16. / 9., image_width, 100, 50));

    let start = Instant::now();
    for i in 0..part_a {
        let cam = Arc::clone(&camera);
        let w = Arc::clone(&world);
        let strings = Arc::clone(&strings);

        let handle = thread::spawn(move || {
            let mut st = strings.lock().unwrap();

            st[i as usize] = cam.render_part(&w, i, part_a);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();

    let strings = strings.lock().unwrap();

    let yay = strings.join("");

    write!(file, "{}", yay).unwrap();

    println!("time to render image: {:?}", duration);
}
