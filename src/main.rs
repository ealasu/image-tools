extern crate regex;

mod image;
mod star_finder;


fn main() {
    let image = image::Image::load("data/a.gray.tif");
    let f = star_finder::StarFinder::new(image);
    let stars = f.find();
    println!("stars: {:?}", stars);
}
