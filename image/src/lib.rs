#[macro_use]
extern crate convert;
extern crate regex;

mod magick;
mod image;
//mod rgb_image;
mod pgm;
mod dcraw;
mod identify;

pub use image::Image;
//pub use rgb_image::*;
pub use identify::*;
