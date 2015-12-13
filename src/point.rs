#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point {
            x: x,
            y: y,
        }
    }
}

#[derive(Debug)]
pub struct IPoint {
    pub x: isize,
    pub y: isize,
}

impl IPoint {
    pub fn new(x: isize, y: isize) -> IPoint {
        IPoint {
            x: x,
            y: y,
        }
    }
}
