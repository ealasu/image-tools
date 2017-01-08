use std::fmt::Debug;
use std::ops::*;
use math::*;
use unit::Unit;
use vector::Vector;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Point<T: Debug + PartialEq + Default> {
    pub x: T,
    pub y: T,
}

//impl Point<Unit> {
    //pub fn is_close_to(self, other: Self, epsilon: f32) -> bool {
        //are_close(self.x, other.x, epsilon) &&
        //are_close(self.y, other.y, epsilon)
    //}
//}

impl Sub for Point<Unit> {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl Div<Unit> for Point<Unit> {
    type Output = Self;

    fn div(self, rhs: Unit) -> Self::Output {
        Point {x: self.x / rhs, y: self.y / rhs}
    }
}

impl Add<Vector> for Point<Unit> {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Point {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl Sub<Vector> for Point<Unit> {
    type Output = Self;

    fn sub(self, rhs: Vector) -> Self::Output {
        Point {x: self.x - rhs.x, y: self.y - rhs.y}
    }
}


