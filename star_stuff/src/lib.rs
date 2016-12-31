#![feature(test)]

extern crate image;
#[cfg(test)] extern crate test;

pub mod point;
pub mod star_stacker;
pub mod types;
pub mod math;

pub use star_stacker::ImageStack;
