#![feature(test)]
#![feature(iter_min_by)]
#![feature(iter_max_by)]

#[macro_use] extern crate convert;
extern crate regex;
extern crate turbojpeg;
extern crate rand;
extern crate byteorder;
extern crate quickersort;
#[cfg(test)] extern crate test;

mod magick;
mod rgb;
mod rgb_bayer;
mod image;
mod image_u8;
mod image_u16;
mod image_f32;
mod image_rgb_u8;
mod image_rgb_f32;
mod image_rgb_bayer;
mod util;
mod pgm;
mod dcraw;
mod identify;
mod fits;

pub use image::*;
pub use rgb::*;
pub use rgb_bayer::*;
pub use identify::*;
