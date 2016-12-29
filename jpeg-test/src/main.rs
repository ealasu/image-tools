extern crate turbojpeg;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = std::env::args().skip(1).next().unwrap();
    let mut jpeg = Vec::new();
    File::open(&filename).unwrap().read_to_end(&mut jpeg).unwrap();
    let image = turbojpeg::decompress(&jpeg[..]);
    println!("{:?}", image);
}
