extern crate regex;

mod image;
mod star_finder;


fn main() {
    let image = image::Image::load("data/a.gray.tif");
    let stars = star_finder::StarFinder::new(&image);
    for star in stars {
        println!("star: {:?}", star);
    }
}
