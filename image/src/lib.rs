#![feature(test)]

#[macro_use] extern crate convert;
extern crate regex;
extern crate turbojpeg;
extern crate rand;
extern crate byteorder;
extern crate quickersort;
extern crate num;
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
mod image_kind;

pub use image::*;
pub use rgb::*;
pub use rgb_bayer::*;
pub use identify::*;
pub use image_kind::*;
