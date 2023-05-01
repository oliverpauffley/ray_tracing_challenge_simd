#![feature(portable_simd)]

use crate::point::Point;
mod point;
mod vector;

fn main() {

    let a = Point::new(1.0, 1.0, 1.0);
    let b = Point::new(1.0, 1.0, 1.0);

    println!("{:?}, {:?}, {}", a,  b, a==b);
}
