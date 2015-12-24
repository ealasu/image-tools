#![feature(test)]

extern crate test;
extern crate regex;
extern crate crossbeam;
extern crate simple_parallel;
extern crate itertools;
extern crate docopt;
extern crate rustc_serialize;

#[macro_use]
mod convert;
mod point;
mod image;
mod star_finder;
mod refine_center;
mod star_aligner;
mod star_stacker;
mod types;
mod math;
mod triangle;

use std::fs;
use std::path::Path;
use simple_parallel::Pool;
use docopt::Docopt;
use point::*;
use types::*;
use image::*;

// steps:
// - find stars
//   needs: list of images
//   (optional: eliminate images with distorted stars)
// - calculate alignment
//   needs: stars
// - stack
//   needs: alignment
//

const USAGE: &'static str = "
Stacker.

Usage:
    stacker <output> <input>...
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_output: String,
    arg_input: Vec<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    //let images = vec![
        //"data/big-1-c.tiff".to_string(),
        //"data/big-2-c.tiff".to_string()
    //];

    //let images = fs::read_dir(Path::new("data/images")).unwrap().map(|f| {
        //f.unwrap()
    //}).filter(|f| {
        //f.metadata().unwrap().is_file()
    //}).map(|f| {
        //f.path().to_str().unwrap().to_string()
    //}).filter(|f| {
        //f.ends_with(".tif") || f.ends_with(".tiff")
    //}).collect::<Vec<_>>();

    //let images = vec![
        //"data/images/IMG_5450.tif".to_string(),
        //"data/images/IMG_5463.tif".to_string(),
    //];

    println!("finding stars");
    let res = find_stars(args.arg_input);
    for (ref f, ref v) in res.iter() {
        println!("found {} stars in {:?}", v.len(), f);
        for star in v.iter() {
            println!("{},{}", star.x, star.y);
        }
    }

    println!("aligning");
    let res = align_images(res);
    println!("aligned:");
    for img in res.iter() {
        println!("{:?}", img);
    }

    println!("stacking");
    star_stacker::stack(&res, &args.arg_output);
}

fn find_stars(images: Vec<String>) -> ImagesWithStars {
    let aperture = 7;
    let mut pool = Pool::new(4);
    crossbeam::scope(|scope| {
        pool.map(scope, &images, |filename| {
            let image = image::Image::open_gray(filename);
            let channel = &image.channels[0];
            let stars = star_finder::StarFinder::new(channel);
            let refined_stars = stars.map(|approx_center| {
                refine_center::refine_star_center(channel, approx_center, aperture)
            }).collect::<Vec<_>>();
            (filename.clone(), refined_stars)
        }).collect()
    })
}

fn align_images(images: ImagesWithStars) -> ImagesWithAlignment {
    let mut pool = Pool::new(4);
    let mut images_iter = (&images).into_iter();
    let (first_image, ref_stars) = images_iter.next().unwrap();
    let mut res = crossbeam::scope(|scope| {
        pool.map(scope, images_iter, |(filename, other_stars)| {
            println!("{}", filename);
            let a = star_aligner::compute_transform(ref_stars, other_stars);
            a.map(|a| {
                (filename.clone(), a)
            })
        }).filter_map(|i| i).collect::<ImagesWithAlignment>()
    });
    res.insert(first_image.clone(), Vector {x: 0.0, y: 0.0});
    res
}
