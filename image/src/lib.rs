#![feature(test)]
#![feature(iter_min_by)]
#![feature(iter_max_by)]

#[macro_use] extern crate convert;
extern crate regex;
extern crate turbojpeg;
extern crate rand;
extern crate byteorder;
#[cfg(test)] extern crate test;

mod magick;
mod image;
//mod rgb_image;
mod pgm;
mod dcraw;
mod identify;
mod fits;

pub use image::{Image, Rgb};
//pub use rgb_image::*;
pub use identify::*;
