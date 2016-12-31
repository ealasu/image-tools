#![feature(test)]

#[macro_use] extern crate convert;
extern crate regex;
extern crate turbojpeg;
#[cfg(test)] extern crate test;
#[cfg(test)] extern crate rand;

mod magick;
mod image;
//mod rgb_image;
mod pgm;
mod dcraw;
mod identify;

pub use image::{Image, Rgb};
//pub use rgb_image::*;
pub use identify::*;
