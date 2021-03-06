use std::ops::*;
use num::Float;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct Vector<T: Float> {
    pub x: T,
    pub y: T,
}

impl<T: Float> Vector<T> {
    #[inline]
    pub fn cross_product(self, other: Self) -> T {
        self.x * other.y - self.y * other.x
    }

    #[inline]
    pub fn length(&self) -> T {
        self.length2().sqrt()
    }

    #[inline]
    pub fn length2(&self) -> T {
        self.x * self.x + self.y * self.y
    }

    #[inline]
    pub fn angle(&self) -> T {
        self.y.atan2(self.x)
    }

    #[inline]
    pub fn floor(&self) -> Self {
        Vector {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }

    //pub fn is_close_to(self, other: Self, epsilon: f32) -> bool {
        //are_close(self.x, other.x, epsilon) &&
        //are_close(self.y, other.y, epsilon)
    //}
}

impl<T: Float> Add for Vector<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Float> Sub for Vector<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Float> Div<T> for Vector<T> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: Float> Mul<T> for Vector<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
