use types::*;


pub fn distance(p1: Star, p2: Star) -> f32 {
    let a = p2.x - p1.x;
    let b = p2.y - p1.y;
    ((a * a) + (b * b)).sqrt()
}

pub fn are_close(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b).abs() < epsilon
}
