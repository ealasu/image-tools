use std::fmt::Debug;
use std::ops::*;


type Unit = f32;


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point<T: Debug + PartialEq> {
    pub x: T,
    pub y: T,
}

impl<T> Sub for Point<T>
where T: Debug + PartialEq + Sub<T>, T::Output: Debug + PartialEq  {
    type Output = Vector<T::Output>;

    fn sub(self, rhs: Point<T>) -> Self::Output {
        Vector {x: self.x - rhs.x, y: self.y - rhs.y}
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector<T: Debug + PartialEq> {
    pub x: T,
    pub y: T,
}

impl<T> Vector<T>
where T: Debug + PartialEq + Mul, <T as Mul>::Output: Sub {
    pub fn cross_product(self, other: Self) -> <<T as Mul>::Output as Sub>::Output {
        self.x * other.y - self.y * other.x
    }

    pub fn length(self) -> 
}
