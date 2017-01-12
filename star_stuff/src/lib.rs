#![feature(test)]

extern crate num;
extern crate image;
extern crate geom;
#[cfg(test)] extern crate test;

pub mod star_stacker;
//pub mod types;
pub mod drizzle;

pub use star_stacker::ImageStack;
