#![feature(test)]

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

pub use image::{Image, Rgb};
//pub use rgb_image::*;
pub use identify::*;
