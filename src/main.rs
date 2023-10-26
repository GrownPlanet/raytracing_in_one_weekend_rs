use std::sync::Arc;
use std::time::Instant;

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

    let mut sampels_per_pixel = 1;
    let max_depth = 4;

    let mut camera = Camera::init(aspect_ratio, image_width, sampels_per_pixel, 50);

    let part_amount = 16;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("raytracer", image_width as u32, image_height as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut current_frame = vec![vec![Color::new(0., 0., 0.); 400]; 255];
    let mut average_frame = vec![vec![Color::new(1., 1., 1.); 400]; 255];

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    camera.move_right();
                    sampels_per_pixel = 1;
                    average_frame = vec![vec![Color::new(1., 1., 1.); 400]; 255];
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    camera.move_left();
                    sampels_per_pixel = 1;
                    average_frame = vec![vec![Color::new(1., 1., 1.); 400]; 255];
                }
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    camera.move_forward();
                    sampels_per_pixel = 1;
                    average_frame = vec![vec![Color::new(1., 1., 1.); 400]; 255];
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    camera.move_back();
                    sampels_per_pixel = 1;
                    average_frame = vec![vec![Color::new(1., 1., 1.); 400]; 255];
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    camera.rotate_right();
                    sampels_per_pixel = 1;
                    average_frame = vec![vec![Color::new(1., 1., 1.); 400]; 255];
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    camera.rotate_left();
                    sampels_per_pixel = 1;
                    average_frame = vec![vec![Color::new(1., 1., 1.); 400]; 255];
                }
                _ => {}
            }
        }

        if sampels_per_pixel > max_depth {
            continue;
        }

        // let start_time = Instant::now();

        let result: Vec<Vec<(i32, i32, Color)>> = (0..part_amount)
            .into_par_iter()
            .map(|i| camera.render_part(&world, i, part_amount))
            .collect();

        // let time_to_trace = start_time.elapsed();
        // let start_time = Instant::now();

        for i in result {
            for c in i {
                let color = c.2.get_color(1.);
                current_frame[c.1 as usize][c.0 as usize] = color;
            }
        }

        average_frame = frag(
            average_frame.clone(),
            current_frame.clone(),
            sampels_per_pixel,
        );

        for y in 0..average_frame.len() {
            for x in 0..average_frame[0].len() {
                let color = average_frame[y][x].clone();
                set_pixel(
                    &mut canvas,
                    x as i32,
                    y as i32,
                    pixels::Color::RGB(color.r as u8, color.g as u8, color.b as u8),
                )?;
            }
        }

        sampels_per_pixel += 1;
        canvas.present();

        // let end_time = start_time.elapsed();
        // println!(
        //     "time to trace: {:?}. time to convert: {:?}",
        //     time_to_trace, end_time
        // );
    }

    Ok(())
}

fn set_pixel(
    canvas: &mut Canvas<Window>,
    x: i32,
    y: i32,
    color: pixels::Color,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.draw_point(Point::new(x, y)).unwrap();

    Ok(())
}

fn frag(
    old_frame: Vec<Vec<Color>>,
    new_frame: Vec<Vec<Color>>,
    iteratioin: i32,
) -> Vec<Vec<Color>> {
    let mut return_vec = vec![vec![Color::new(0., 0., 0.,); 400]; 255];

    let weight = 1. / (iteratioin as f64 + 1.);

    for y in 0..old_frame.len() {
        for x in 0..old_frame[0].len() {
            return_vec[y][x] =
                old_frame[y][x].clone() * (1. - weight) + new_frame[y][x].clone() * weight;
        }
    }

    return_vec
}
