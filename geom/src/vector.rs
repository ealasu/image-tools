use std::ops::*;
use unit::Unit;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct Vector {
    pub x: Unit,
    pub y: Unit,
}

impl Vector {
    pub fn cross_product(self, other: Self) -> Unit {
        self.x * other.y - self.y * other.x
    }

    pub fn length(&self) -> Unit {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn angle(&self) -> Unit {
        self.x.atan2(self.y)
    }

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

impl Add for Vector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Div<Unit> for Vector {
    type Output = Self;
    fn div(self, rhs: Unit) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Mul<Unit> for Vector {
    type Output = Self;
    fn mul(self, rhs: Unit) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
