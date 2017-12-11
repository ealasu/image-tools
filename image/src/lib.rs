#![feature(decl_macro)]

extern crate convert;
extern crate regex;
//extern crate turbojpeg;
extern crate rand;
extern crate byteorder;
extern crate quickersort;
extern crate num;
//extern crate fits;
//extern crate imagemagick;
//#[cfg(test)] extern crate test;

mod rgb;
mod rgb_bayer;
mod image;
//mod image_u8;
mod image_f32;
//mod image_rgb_u8;
//mod image_rgb_f32;
//mod image_rgb_bayer;
mod util;
//mod pgm;
//mod dcraw;
//mod image_kind;

pub use image::*;
pub use rgb::*;
pub use rgb_bayer::*;
//pub use image_kind::*;
