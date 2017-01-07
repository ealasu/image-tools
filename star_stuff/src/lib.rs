#![feature(test)]

extern crate image;
#[cfg(test)] extern crate test;

pub mod point;
pub mod star_stacker;
pub mod types;
pub mod math;
pub mod drizzle;
pub mod stack;

pub use star_stacker::ImageStack;
