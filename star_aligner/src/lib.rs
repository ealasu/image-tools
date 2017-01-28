#![feature(conservative_impl_trait)]

extern crate sextractor;
extern crate geom;
extern crate simd;
#[cfg(test)] extern crate serde_json;

use geom::{Point, Matrix3x3, Matrix3x1};
use simd::f32x4;

#[derive(Debug, Clone)]
pub struct Polygon {
    sides: f32x4,
    stars: [Point<f32>; 3],
}

impl Polygon {
    pub fn shift(&self, amount: usize) -> Self {
        match amount {
            0 => self.clone(), 
            1 => Polygon {
                sides: f32x4::new(
                           self.sides.extract(1),
                           self.sides.extract(2),
                           self.sides.extract(0),
                           0.0),
                stars: [self.stars[1], self.stars[2], self.stars[0]],
            },
            2 => Polygon {
                sides: f32x4::new(
                           self.sides.extract(1),
                           self.sides.extract(2),
                           self.sides.extract(0),
                           0.0),
                stars: [self.stars[1], self.stars[2], self.stars[0]],
            },
            _ => unimplemented!()
        }
    }
}

pub fn extract(path: &str) -> Vec<Point<f32>> {
    let mut objects = sextractor::extract(path);
    // sort by flux, descending
    objects.sort_by(|a,b| b.flux.partial_cmp(&a.flux).unwrap());
    objects
        .into_iter()
        .take(300)
        .map(|o| Point { x: o.x, y: o.y })
        .collect()
}

fn get_transform_matrix(ref_p: &Polygon, sam_p: &Polygon) -> Matrix3x3<f64> {
    println!("ref_p: {:?} sam_p: {:?}", ref_p, sam_p);
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

            //v11: p.stars[0].x,
            //v21: p.stars[0].y,
            //v31: 1.0,
            //v12: p.stars[1].x,
            //v22: p.stars[1].y,
            //v32: 1.0,
            //v13: p.stars[2].x,
            //v23: p.stars[2].y,
            //v33: 1.0,
        }.to_f64()
    }
    let res = poly_to_matrix(sam_p).inverse() * poly_to_matrix(ref_p);
    if res.has_nan() {
        panic!("matrix has nan: {:?}", res);
    }

    println!("sam_p:    {:?}", poly_to_matrix(sam_p));
    println!("sam_p tx: {:?}", poly_to_matrix(sam_p) * res);
    println!("ref_p:    {:?}", poly_to_matrix(ref_p));
    let sam_star = Matrix3x3 {
        v11: sam_p.stars[1].x as f64,
        v12: sam_p.stars[1].y as f64,
        v13: 1.0,

        v21: 0.0,
        v22: 1.0,
        v23: 0.0,

        v31: 0.0,
        v32: 0.0,
        v33: 1.0,
    };
    //println!("sam star:    {:?}", Matrix3x1::from_point(&sam_p.stars[1]).to_f64());
    println!("sam star:    {:?}", sam_star);
    //println!("ref star:    {:?}", Matrix3x1::from_point(&ref_p.stars[1]).to_f64());
    println!("ref star:    {:?}", ref_p.stars[1].to_f64());
    println!("sam star tx: {:?}", sam_star * res);

    res
}

pub fn align_images(ref_image: &str, sample_image: &str) -> Matrix3x3<f64> {
    align_stars(extract(ref_image), extract(sample_image))
}

pub fn align_stars(ref_objects: Vec<Point<f32>>, sample_objects: Vec<Point<f32>>) -> Matrix3x3<f64> {
    let ref_polys = polys(&ref_objects[..]).collect::<Vec<_>>();
    //for v in ref_polys[..12].iter() {
        //println!("ref: {:?}", v.sides);
    //}
    //for (a,b) in ref_polys.iter().zip(sample_polys.iter()).take(20) {
        //println!("ref: {:?}", a.sides);
        //println!("sam: {:?}", b.sides);
        //println!(" ");
    //}

    let mut matches = vec![];

    let threshold = 0.3;
    let threshold_lower = f32x4::splat(-threshold);
    let threshold_upper = f32x4::splat(threshold);
    for sam_p in polys(&sample_objects[..]) {
        for sam_p in [
            sam_p.shift(0),
            sam_p.shift(1),
            sam_p.shift(2),
        ].iter() {
            for ref_p in ref_polys.iter() {
                let diff = sam_p.sides - ref_p.sides;
                if diff.gt(threshold_lower).all() && diff.lt(threshold_upper).all() {
                    //let d = (diff.extract(0) + diff.extract(1) + diff.extract(2)) / 3.0;
                    matches.push((ref_p.clone(), sam_p.clone()));

                    let m = get_transform_matrix(ref_p, &sam_p);
                    println!("m: {:?}", m);

                    let proof = sample_objects
                        .iter()
                        .filter_map(|&s_o| {
                            let tx = (m * Matrix3x1::from_point(&s_o.to_f64())).to_point().to_f32();
                            println!("src: {:?}  tx: {:?}", s_o, tx);
                            ref_objects
                                .iter()
                                .find(|&r_o| r_o.is_close_to(s_o, 1.0))
                        })
                        .take(100)
                        .collect::<Vec<_>>();
                    println!("proof: {}", proof.len());
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
    use serde_json;
    use std::fs::File;

    fn read_stars(filename: &str) -> Vec<Point<f32>> {
        let mut f = File::open(filename).unwrap();
        serde_json::from_reader(&mut f).unwrap()
    }

    fn write_stars(src: &str, dst: &str) {
        let mut f = File::create(dst).unwrap();
        serde_json::to_writer(&mut f, &extract(src)).unwrap();
    }

    //#[test]
    fn gen_data() {
        write_stars("test/a.fits", "test/a.stars.json");
        write_stars("test/b.fits", "test/b.stars.json");
    }

    #[test]
    fn test_align() {
        let res = align_stars(
            read_stars("test/a.stars.json"),
            read_stars("test/b.stars.json"));
    }

}
