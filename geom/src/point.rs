use std::ops::*;
//use math::*;
use vector::Vector;
use num::Float;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

//impl Point<T> {
    //pub fn is_close_to(self, other: Self, epsilon: f32) -> bool {
        //are_close(self.x, other.x, epsilon) &&
        //are_close(self.y, other.y, epsilon)
    //}
//}

impl<T: Float> Sub for Point<T> {
    type Output = Vector<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl<T: Float> Div<T> for Point<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Point {x: self.x / rhs, y: self.y / rhs}
    }
}

impl<T: Float> Add<Vector<T>> for Point<T> {
    type Output = Self;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Point {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl<T: Float> Sub<Vector<T>> for Point<T> {
    type Output = Self;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Point {x: self.x - rhs.x, y: self.y - rhs.y}
    }
}
