use std::fmt::Debug;


#[derive(Debug, PartialEq)]
pub struct Point<T: Debug + PartialEq> {
    pub x: T,
    pub y: T,
}
