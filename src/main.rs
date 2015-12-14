#![feature(test)]

extern crate test;
extern crate regex;

mod point;
mod spiral;
mod image;
mod star_finder;
mod refine_center;


fn main() {
    let image = image::Image::load("data/a.gray.tif");
    let stars = star_finder::StarFinder::new(&image);
    for star in stars {
        println!("star: {:?}", star);
    }
}
