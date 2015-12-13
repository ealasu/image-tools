extern crate regex;

mod image;


fn main() {
    let im = image::Image::load("data/a.gray.tif");

    println!("max: {}", im.pixels().iter().max().unwrap());
    println!("min: {}", im.pixels().iter().min().unwrap());

    let average: f64 = im.pixels().iter().map(|&v| v as f64).fold(0f64, |sum, i| sum + i) /
        im.pixels().len() as f64;
    println!("average: {}", average);

    for i in 1..10 {
        println!("at {},8: {}", i, im.at(i, 8));
    }
}
