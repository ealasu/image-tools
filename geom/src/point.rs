use std::ops::*;
//use math::*;
use vector::Vector;
use num::Float;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: Float> Point<T> {
    pub fn is_close_to(self, other: Self, threshold: T) -> bool {
        let are_close = |a: T, b: T| {
            let d = a - b;
            d < threshold && d > -threshold
        };
        are_close(self.x, other.x) &&
        are_close(self.y, other.y)
    }
}

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
