#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate geom;

use std::fs::File;
use std::io::prelude::*;
use geom::Matrix3x3;

#[derive(Serialize, Deserialize, Debug)]
pub struct AlignedImage {
    pub filename: String,
    pub transform: Matrix3x3<f32>,
}

pub fn write(alignment: &[AlignedImage], filename: &str) {
    let mut file = File::create(&filename).unwrap();
    let json = serde_json::to_string(&alignment).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

pub fn read(filename: &str) -> Vec<AlignedImage> {
    let mut file = File::open(&filename).unwrap();
    let mut json = String::new();
    file.read_to_string(&mut json).unwrap();
    serde_json::from_str(&json).unwrap()
}
