use std::{
    fs::{self, File},
    io::prelude::*,
    path::Path,
};

pub mod color;
pub mod point3;

use crate::color::Color;

fn main() {
    // open file to put image in
    let path = "image.ppm";
    if Path::new(path).exists() {
        fs::remove_file(path).unwrap();
    }

    let mut file = File::create(path).unwrap();

    // image dimensions

    let image_width = 256;
    let image_height = 256;

    // write basic ppm info to file
    write!(file, "P3\n{} {}\n255\n", image_width, image_height).unwrap();

    for j in 0..image_height {
        println!("Scanlines remaining: {}", (image_height - j));
        for i in 0..image_width {
            let pixel_color = Color::new(j as f64, 0., i as f64);

            write!(file, "{}", pixel_color.to_string()).unwrap();
        }
    }
    println!("-------------- Done --------------")
}
