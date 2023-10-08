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

    let mut file = File::create(path);

    println!("Hello, world!");
}
