use randerive::{Rand};
use rand::distributions::{Standard, Distribution};

#[derive(Debug, Rand)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T
}

#[derive(Debug, Rand)]
pub struct Mat2<T> {
    pub x: Vec2<T>,
    pub y: Vec2<T>
}

fn main () {
    let test : Mat2<f64> = rand::random();
    print!("{:?}", test)
}