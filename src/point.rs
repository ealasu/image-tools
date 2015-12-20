use std::fmt::Debug;
use std::ops::*;


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point<T: Debug + PartialEq> {
    pub x: T,
    pub y: T,
}

impl<T> Sub for Point<T>
where T: Debug + PartialEq + Sub<T>, T::Output: Debug + PartialEq  {
    type Output = Point<T::Output>;

    fn sub(self, rhs: Point<T>) -> Self::Output {
        Point {x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl<T> Point<T>
where T: Debug + PartialEq + Mul, <T as Mul>::Output: Sub {
    pub fn cross_product(self, other: Point<T>) -> <<T as Mul>::Output as Sub>::Output {
        self.x * other.y - self.y * other.x
    }
}
