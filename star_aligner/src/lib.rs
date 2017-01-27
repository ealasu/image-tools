#![feature(conservative_impl_trait)]

extern crate sextractor;
extern crate geom;
extern crate simd;

use sextractor::Object;
use geom::{Point, Matrix3x3};
use simd::f32x4;

#[derive(Debug, Clone)]
pub struct Polygon {
    sides: f32x4,
    stars: [Point<f32>; 3],
}

pub fn align(ref_image: &str, sample_image: &str) -> Matrix3x3<f64> {
    let ref_polys = polys(sextractor::extract(ref_image));
    //for v in ref_polys[..12].iter() {
        //println!("ref: {:?}", v.sides);
    //}
    let sample_polys = polys(sextractor::extract(sample_image));
    //for v in sample_polys[..12].iter() {
        //println!("sam: {:?}", v.sides);
    //}

    //for (a,b) in ref_polys.iter().zip(sample_polys.iter()).take(20) {
        //println!("ref: {:?}", a.sides);
        //println!("sam: {:?}", b.sides);
        //println!(" ");
    //}

    let mut matches = vec![];

    let threshold = 1.0;
    let threshold_lower = f32x4::splat(-threshold);
    let threshold_upper = f32x4::splat(threshold);
    for sam_p in sample_polys.iter() {
        for &sam_p_sides in [
            sam_p.sides,
            f32x4::new(sam_p.sides.extract(1), sam_p.sides.extract(2), sam_p.sides.extract(0), 0.0),
            f32x4::new(sam_p.sides.extract(2), sam_p.sides.extract(0), sam_p.sides.extract(1), 0.0),

            //f32x4::new(sam_p.sides.extract(2), sam_p.sides.extract(1), sam_p.sides.extract(0), 0.0),
            //f32x4::new(sam_p.sides.extract(1), sam_p.sides.extract(0), sam_p.sides.extract(2), 0.0),
            //f32x4::new(sam_p.sides.extract(0), sam_p.sides.extract(2), sam_p.sides.extract(1), 0.0),
        ].iter() {
            for ref_p in ref_polys.iter() {
                let diff = sam_p_sides - ref_p.sides;
                if diff.gt(threshold_lower).all() && diff.lt(threshold_upper).all() {
                    let d = (diff.extract(0) + diff.extract(1) + diff.extract(2)) / 3.0;
                    matches.push((ref_p.clone(), sam_p.clone(), d));
                }
            }
        }
    }

    println!("matches: {}", matches.len());
    //for v in matches[..4].iter() {
        //println!("match: {:?}", v);
    //}
    unimplemented!();
}

pub fn angle(stars: &[Point<f32>]) -> f32 {
    assert_eq!(stars.len(), 3);
    (stars[2] - stars[0]).angle() - (stars[1] - stars[0]).angle()
}

fn sliding_window<'a, T, F>(v: &'a [T], size: usize, mut f: F) where F: FnMut(&'a[T]) {
    for i in 0..v.len() - size {
        f(&v[i..i+size])
    }
}

fn polys(mut objects: Vec<Object>) -> Vec<Polygon> {
    println!("stars detected: {}", objects.len());
    assert!(objects.len() > 2);

    // reverse sort by flux
    objects.sort_by(|a,b| b.flux.partial_cmp(&a.flux).unwrap());
    objects.truncate(300);

    let mut res = Vec::with_capacity(objects.len() - 2);
    {
        let mut push_poly = |window: [&Object; 3]| {
            let mut stars = window
                .iter()
                .take(3)
                .map(|obj| Point { x: obj.x, y: obj.y })
                .collect::<Vec<_>>();

            // make sure all triangles are clockwise
            let poly_angle = angle(&stars[..]);
            //println!("angle: {}", poly_angle);
            if poly_angle < 0.0 {
                stars.reverse();
            }

            res.push(Polygon {
                sides: f32x4::new(
                    (stars[1] - stars[0]).length(),
                    (stars[2] - stars[1]).length(),
                    (stars[0] - stars[2]).length(),
                    0.0),
                stars: [stars[0], stars[1], stars[2]],
            });
        };

        sliding_window(&objects[..], 3, |window| {
            push_poly([&window[0], &window[1], &window[2]]);
        });
        sliding_window(&objects[..], 4, |window| {
            push_poly([&window[0], &window[1], &window[3]]);
            push_poly([&window[0], &window[2], &window[3]]);
        });
        sliding_window(&objects[..], 5, |window| {
            push_poly([&window[0], &window[1], &window[4]]);
            push_poly([&window[0], &window[2], &window[4]]);
            push_poly([&window[0], &window[3], &window[4]]);
        });
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = align("test/a.fits", "test/b.fits");
    }
}
