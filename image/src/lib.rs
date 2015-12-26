#[macro_use]
extern crate convert;
extern crate regex;

mod channel;
mod image;
mod magick;
mod gray_image;
mod rgb_image;
mod pgm;
mod dcraw;
mod identify;

pub use channel::*;
pub use image::*;
pub use gray_image::*;
pub use rgb_image::*;
pub use identify::*;
