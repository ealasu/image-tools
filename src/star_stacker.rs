use std::cmp::*;
use std::collections::BTreeMap;
use image::Image;
use point::Vector;


struct ImageStack {
    image: Image,
}

impl ImageStack {
    pub fn new(width: usize, height: usize) -> ImageStack {
        ImageStack {
            image: Image::new(width, height),
        }
    }

    pub fn add(&mut self, image: &Image, transform: Vector) {

    }

    pub fn to_image(self) -> Image {
        self.image
    }
}

pub fn stack(images: &BTreeMap<String, Vector>) -> Image {
    let d = images.iter().map(|(filename, &tx)| {
        let (width, height) = Image::identify(&filename);
        let top = tx.y as isize;
        let bottom = height as isize + tx.y as isize;
        let left = tx.x as isize;
        let right = width as isize + tx.x as isize;
        (top, right, bottom, left)
    }).collect::<Vec<_>>();
    let right = d.iter().map(|&(_, right, _, _)| right).min().unwrap();
    let left =  d.iter().map(|&(_, _, _, left)| left).max().unwrap();
    let width = max(0, right - left) as usize;
    let bottom = d.iter().map(|&(_, _, bottom, _)| bottom).min().unwrap();
    let top = d.iter().map(|&(top, _, _, _)| top).max().unwrap();
    let height = max(0, bottom - top) as usize;
    assert!(width > 0);
    assert!(height > 0);
    let mut stack = ImageStack::new(width, height);
    for (filename, &tx) in images.iter() {
        let image = Image::load(filename);
        stack.add(&image, tx);
    }
    stack.to_image()
}
