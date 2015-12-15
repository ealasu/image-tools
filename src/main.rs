#![feature(test)]

extern crate test;
extern crate regex;

mod point;
mod spiral;
mod image;
mod star_finder;
mod refine_center;


fn main() {
    let image = image::Image::load("data/a.gray.tiff");
    let stars = star_finder::StarFinder::new(&image);
    for approx_center in stars {
        let center = refine_center::refine_star_center(&image, approx_center, 7);
        println!("star: {:?}", center);
    }
}
