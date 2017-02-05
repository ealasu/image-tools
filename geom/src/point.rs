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
    #[inline]
    pub fn to_f64(&self) -> Point<f64> {
        Point {
            x: self.x.to_f64().unwrap(),
            y: self.y.to_f64().unwrap(),
        }
    }

    #[inline]
    pub fn to_f32(&self) -> Point<f32> {
        Point {
            x: self.x.to_f32().unwrap(),
            y: self.y.to_f32().unwrap(),
        }
    }

    #[inline]
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

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vector {x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl<T: Float> Div<T> for Point<T> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Point {x: self.x / rhs, y: self.y / rhs}
    }
}

impl<T: Float> AddAssign for Point<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<T: Float> Add<Vector<T>> for Point<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Vector<T>) -> Self::Output {
        Point {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl<T: Float> Sub<Vector<T>> for Point<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Point {x: self.x - rhs.x, y: self.y - rhs.y}
    }
}
