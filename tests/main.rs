use randerive::{Rand};

#[derive(Debug, Rand)]
struct Vec2<T> {
    x: T,
    y: T
}

fn main () {
    let test : Vec2<f64> = rand::random();
    print!("{:?}", test)
}