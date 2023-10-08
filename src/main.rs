use std::{
    fs::{self, File},
    io::prelude::*,
    path::Path,
};

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
    write!(file, "P3\n {} {}\n255\n", image_width, image_height).unwrap();

    for j in 0..image_height {
        for i in 0..image_width {
            let r = j;
            let g = 0;
            let b = i;

            write!(file, "{} {} {}\n", r, g, b).unwrap();
        }
    }
}
