#![feature(test)]

extern crate test;
extern crate regex;
extern crate crossbeam;
extern crate simple_parallel;

mod point;
mod spiral;
mod image;
mod star_finder;
mod refine_center;

use simple_parallel::Pool;

// steps:
// - find stars
//   needs: list of images
//   (optional: eliminate images with distorted stars)
// - calculate alignment
//   needs: stars
// - stack
//   needs: alignment
//

//struct ImageWithStars {
    //file: String,
    //stars: Vec<Point<f32>>,
//}

//struct ImageWithAlignment {
    //file: String,
    //transform: Point<f32>,
//}

fn main() {
    let image_files = vec!["data/big-1.tiff", "data/big-2.tiff"];

    let mut pool = Pool::new(4);

    crossbeam::scope(|scope| {
        let res = pool.map(scope, &image_files, |&filename| {
            let image = image::Image::load(filename);
            let stars = star_finder::StarFinder::new(&image);
            let refined_stars = stars.map(|approx_center| {
                refine_center::refine_star_center(&image, approx_center, 7)
            }).collect::<Vec<_>>();
            (filename.clone(), refined_stars)
        }).collect::<Vec<_>>();

        for (f, v) in res {
            println!("found {} stars in {:?}", v.len(), f);
        }
    });
}
