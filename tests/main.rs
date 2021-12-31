use randerive::{Rand};

#[derive(Debug, Rand)]
struct Vec2 {
    x: f32,
    y: f32
}

fn main () {
    let test : Vec2 = rand::random();
    print!("{:?}", test)
}