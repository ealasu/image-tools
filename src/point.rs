use std::fmt::Debug;
use std::ops::*;


pub type Unit = f32;


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point<T: Debug + PartialEq> {
    pub x: T,
    pub y: T,
}

impl Sub for Point<Unit> {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {x: self.x - rhs.x, y: self.y - rhs.y}
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Vector {
    pub x: Unit,
    pub y: Unit,
}

impl Vector {
    pub fn cross_product(self, other: Self) -> Unit {
        self.x * other.y - self.y * other.x
    }

    pub fn length(self) -> Unit {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
