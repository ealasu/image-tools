#![feature(test)]

extern crate test;
extern crate regex;
extern crate crossbeam;
extern crate simple_parallel;
extern crate itertools;
extern crate convert;
extern crate image;

#[macro_use]
pub mod point;
pub mod star_finder;
pub mod refine_center;
pub mod star_aligner;
pub mod star_stacker;
pub mod types;
pub mod math;
pub mod triangle;

pub use star_finder::find_stars;
pub use star_aligner::align_images;
pub use star_stacker::stack;