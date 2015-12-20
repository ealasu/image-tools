#![feature(test)]

extern crate test;
extern crate regex;
extern crate crossbeam;
extern crate simple_parallel;
extern crate itertools;

mod point;
mod spiral;
mod image;
mod star_finder;
mod refine_center;
mod star_aligner;
mod types;

use std::collections::HashMap;
use simple_parallel::Pool;
use point::Point;
use types::*;

// steps:
// - find stars
//   needs: list of images
//   (optional: eliminate images with distorted stars)
// - calculate alignment
//   needs: stars
// - stack
//   needs: alignment
//


fn main() {
    let images = vec!["data/big-1.tiff".to_string(), "data/big-2.tiff".to_string()];
    let res = find_stars(images);
    //for (ref f, ref v) in res.iter() {
        //println!("found {} stars in {:?}", v.len(), f);
        //for star in v.iter() {
            //println!("{},{}", star.x, star.y);
        //}
    //}
    let res = align_images(res);
    let res = stack_images(res);
    // TODO: save res
}

fn find_stars(images: Vec<String>) -> ImagesWithStars {
    let aperture = 7;
    let mut pool = Pool::new(4);
    crossbeam::scope(|scope| {
        pool.map(scope, &images, |filename| {
            let image = image::Image::load(filename);
            let stars = star_finder::StarFinder::new(&image);
            let refined_stars = stars.map(|approx_center| {
                refine_center::refine_star_center(&image, approx_center, aperture)
            }).collect::<Vec<_>>();
            (filename.clone(), refined_stars)
        }).collect()
    })
}

fn align_images(images: ImagesWithStars) -> ImagesWithAlignment {
    let mut pool = Pool::new(1);
    let mut images_iter = (&images).into_iter();
    let (first_image, ref_stars) = images_iter.next().unwrap();
    let mut res = crossbeam::scope(|scope| {
        pool.map(scope, images_iter, |(filename, other_stars)| {
            println!("{}", filename);
            (filename.clone(), star_aligner::compute_transform(ref_stars, other_stars))
        }).collect::<ImagesWithAlignment>()
    });
    res.insert(first_image.clone(), Point {x: 0.0, y: 0.0});
    res
}

fn stack_images(images: ImagesWithAlignment) {
}

