#![feature(conservative_impl_trait)]

extern crate sextractor;
extern crate geom;
extern crate simd;

use geom::{Point, Matrix3x3};
use simd::f32x4;

#[derive(Debug, Clone)]
pub struct Polygon {
    sides: f32x4,
    stars: [Point<f32>; 3],
}

fn extract(path: &str) -> Vec<Point<f32>> {
    let mut objects = sextractor::extract(path);
    objects.sort_by(|a,b| b.flux.partial_cmp(&a.flux).unwrap());
    objects
        .into_iter()
        .take(300)
        .map(|o| Point { x: o.x, y: o.y })
        .collect()
}

fn get_transform_matrix(ref_p: &Polygon, sam_p: &Polygon) -> Matrix3x3<f64> {
    fn poly_to_matrix(p: &Polygon) -> Matrix3x3<f64> {
        Matrix3x3 {
            v11: p.stars[0].x,
            v12: p.stars[0].y,
            v13: 1.0,
            v21: p.stars[1].x,
            v22: p.stars[1].y,
            v23: 1.0,
            v31: p.stars[2].x,
            v32: p.stars[2].y,
            v33: 1.0,
        }.to_f64()
    }
    let res = poly_to_matrix(sam_p).inverse() * poly_to_matrix(ref_p);
    if res.has_nan() {
        panic!("matrix has nan: {:?}", res);
    }
    res
}

pub fn align(ref_image: &str, sample_image: &str) -> Matrix3x3<f64> {
    let ref_objects = extract(ref_image);
    let ref_polys = polys(&ref_objects[..]).collect::<Vec<_>>();
    //for v in ref_polys[..12].iter() {
        //println!("ref: {:?}", v.sides);
    //}
    let sample_objects = extract(sample_image);

    //for (a,b) in ref_polys.iter().zip(sample_polys.iter()).take(20) {
        //println!("ref: {:?}", a.sides);
        //println!("sam: {:?}", b.sides);
        //println!(" ");
    //}

    let mut matches = vec![];

    let threshold = 0.4;
    let threshold_lower = f32x4::splat(-threshold);
    let threshold_upper = f32x4::splat(threshold);
    for sam_p in polys(&sample_objects[..]) {
        for &sam_p_sides in [
            sam_p.sides,
            f32x4::new(sam_p.sides.extract(1), sam_p.sides.extract(2), sam_p.sides.extract(0), 0.0),
            f32x4::new(sam_p.sides.extract(2), sam_p.sides.extract(0), sam_p.sides.extract(1), 0.0),
        ].iter() {
            for ref_p in ref_polys.iter() {
                let diff = sam_p_sides - ref_p.sides;
                if diff.gt(threshold_lower).all() && diff.lt(threshold_upper).all() {
                    //let d = (diff.extract(0) + diff.extract(1) + diff.extract(2)) / 3.0;
                    matches.push((ref_p.clone(), sam_p.clone()));

                    let m = get_transform_matrix(ref_p, &sam_p);

                }
            }
        }
    }

    println!("matches: {}", matches.len());
    for v in matches[..4].iter() {
        println!("match: {:?}", v);
    }
    unimplemented!();
}

pub fn angle(stars: &[Point<f32>]) -> f32 {
    assert_eq!(stars.len(), 3);
    (stars[2] - stars[0]).angle() - (stars[1] - stars[0]).angle()
}

fn polys<'a>(objects: &'a [Point<f32>]) -> impl Iterator<Item=Polygon> + 'a {
    println!("stars detected: {}", objects.len());
    assert!(objects.len() > 2);

    fn make_poly(window: [&Point<f32>; 3]) -> Polygon {
        let mut stars = window
            .iter()
            .take(3)
            .map(|x| **x)
            .collect::<Vec<_>>();

        // make sure all triangles are clockwise
        let poly_angle = angle(&stars[..]);
        //println!("angle: {}", poly_angle);
        if poly_angle < 0.0 {
            stars.reverse();
        }

        Polygon {
            sides: f32x4::new(
                (stars[1] - stars[0]).length(),
                (stars[2] - stars[1]).length(),
                (stars[0] - stars[2]).length(),
                0.0),
            stars: [stars[0], stars[1], stars[2]],
        }
    }

    objects.windows(3).map(|window| {
        make_poly([&window[0], &window[1], &window[2]])
    }).chain(
    objects.windows(4).map(|window| {
        make_poly([&window[0], &window[1], &window[3]])
    })).chain(
    objects.windows(4).map(|window| {
        make_poly([&window[0], &window[2], &window[3]])
    })).chain(
    objects.windows(5).map(|window| {
        make_poly([&window[0], &window[1], &window[4]])
    })).chain(
    objects.windows(5).map(|window| {
        make_poly([&window[0], &window[2], &window[4]])
    })).chain(
    objects.windows(5).map(|window| {
        make_poly([&window[0], &window[3], &window[4]])
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = align("test/a.fits", "test/b.fits");
    }
}
