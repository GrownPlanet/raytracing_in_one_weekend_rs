use std::sync::Arc;

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
use sdl2::rect::Rect;
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

    let sampels_per_pixel = 1;

    let camera = Camera::init(aspect_ratio, image_width, sampels_per_pixel, 50);

    let part_amount = 16;

    let multiplier = 2;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "raytracer",
            image_width as u32 * multiplier,
            image_height as u32 * multiplier,
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
                let color = c.2.get_color(sampels_per_pixel as f64);
                set_pixel(
                    &mut canvas,
                    c.0,
                    c.1,
                    pixels::Color::RGB(color.r as u8, color.g as u8, color.b as u8),
                    multiplier as i32,
                )?;
            }
        }

        // canvas.clear();
        canvas.present();
    }
    Ok(())
}

fn set_pixel(
    canvas: &mut Canvas<Window>,
    x: i32,
    y: i32,
    color: pixels::Color,
    m: i32,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    // canvas.draw_point(Point::new(x, y)).unwrap();
    canvas.fill_rect(Rect::new(
        x * m,
        y * m,
        ((x + 1) * m) as u32,
        ((y + 1) * m) as u32,
    ))?;

    Ok(())
}
