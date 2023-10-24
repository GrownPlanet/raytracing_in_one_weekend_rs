use std::{
    // fs::{self, File},
    // io::prelude::*,
    path::Path,
    sync::Arc,
    // time::Instant,
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

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

fn main() -> Result<(), String> {
    // open file to put image in
    // let path = "image.ppm";
    // if Path::new(path).exists() {
    //     fs::remove_file(path).unwrap();
    // }

    // let mut file = File::create(path).unwrap();

    // world
    let world = Arc::new(HittableList::new(vec![
        Box::new(Sphere::new(
            Point3::new(0., 0., -1.),
            0.5,
            Arc::new(Lambertian::new(Color::new(0.3, 0.1, 0.7))),
        )),
        Box::new(Sphere::new(
            Point3::new(1., 0., -1.),
            0.5,
            Arc::new(Metal::new(Color::new(0.7, 0.2, 0.4), 0.5)),
        )),
        Box::new(Sphere::new(
            Point3::new(-1., 0., -1.),
            0.5,
            Arc::new(Metal::new(Color::new(0.5, 0.5, 0.5), 0.2)),
        )),
        Box::new(Sphere::new(
            Point3::new(0., -100.5, -1.),
            100.,
            Arc::new(Lambertian::new(Color::new(0.3, 0.6, 0.1))),
        )),
    ]));

    let aspect_ratio = 16. / 9.;
    let image_width = 400;
    let image_height = image_width as f64 / aspect_ratio;

    let camera = Camera::init(aspect_ratio, image_width, 1, 50);

    let part_amount = 16;

    // let start = Instant::now();

    // let result: Vec<String> = (0..part_amount)
    //     .into_par_iter()
    //     .map(|i| {
    //         // f
    //         camera.render_part(&world, i, part_amount)
    //     })
    //     .collect();

    // let duration = start.elapsed();
    // println!("time to render image: {:?}", duration);

    // let result_string = result.join("");

    // write!(file, "P3\n{} {}\n255\n", image_width, image_height as i32).unwrap();
    // write!(file, "{}", result_string).unwrap();

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "rust-sdl2 demo: Video",
            image_width as u32,
            image_height as u32,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        let result: Vec<Vec<(i32, i32, Color)>> = (0..part_amount)
            .into_par_iter()
            .map(|i| {
                // f
                camera.render_part(&world, i, part_amount)
            })
            .collect();

        for i in result {
            for c in i {
                set_pixel(
                    &mut canvas,
                    c.0,
                    c.1,
                    pixels::Color::RGB(
                        (c.2.r * 256.) as u8,
                        (c.2.g * 256.) as u8,
                        (c.2.b * 256.) as u8,
                    ),
                );
            }
        }

        // canvas.clear();
        canvas.present();
    }
    Ok(())
}

fn set_pixel(canvas: &mut Canvas<Window>, x: i32, y: i32, color: pixels::Color) {
    canvas.set_draw_color(color);
    canvas.draw_point(Point::new(x, y)).unwrap();
}
