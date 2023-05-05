#![feature(portable_simd)]

use std::{fs::File, path::Path};

use canvas::Canvas;

use crate::point::Point;
mod canvas;
mod color;
mod point;
mod vector;

/// compares if two float values are roughly equivalent
pub fn float_equal(a: f64, b: f64) -> bool {
    a - b < f64::EPSILON / 2.0
}

fn main() {
    let canvas = Canvas::new(200, 200);

    let file_path = Path::new(r"./images/test.png");
    let file = File::create(file_path).unwrap();

    canvas.write_file(file).unwrap()
}
