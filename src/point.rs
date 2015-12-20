use std::fmt::Debug;
use std::ops::Sub;


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point<T: Debug + PartialEq> {
    pub x: T,
    pub y: T,
}

impl<T> Sub for Point<T>
where T: Sub<T>, T::Output: Debug + PartialEq  {
    type Output = Point<T::Output>;

    fn sub(self, rhs: Point<T>) -> Self::Output {
        Point {x: self.x - rhs.x, y: self.y - rhs.y}
    }
}
