#![feature(conservative_impl_trait)]
#![feature(test)]

extern crate sextractor;
extern crate geom;
extern crate simd;
#[cfg(test)] extern crate serde_json;
#[cfg(test)] extern crate test;

use geom::{Point, Matrix3x3, Matrix3x1};
use simd::f32x4;

const N_OBJECTS: usize = 400;
const N_PROOF: usize = 300;

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

pub struct Reference {
    stars: Vec<Point<f32>>,
    polys: Vec<Polygon>,
}

impl Reference {
    pub fn from_image(path: &str) -> Self {
        Self::from_stars(extract(path))
    }

    pub fn from_stars(stars: Vec<Point<f32>>) -> Self {
        let polys = polys(&stars).collect();
        Reference {
            polys: polys,
            stars: stars,
        }
    }

    pub fn align_image(&self, sample: &str) -> Option<Matrix3x3<f64>> {
        self.align_stars(&extract(sample))
    }

    pub fn align_stars(&self, sample_objects: &[Point<f32>]) -> Option<Matrix3x3<f64>> {
        let threshold = 0.4;

        let threshold_lower = f32x4::splat(-threshold);
        let threshold_upper = f32x4::splat(threshold);
        for sam_p in polys(sample_objects) {
            for sam_p in [
                sam_p.shift(0),
                sam_p.shift(1),
                sam_p.shift(2),
            ].iter() {
                for ref_p in self.polys.iter() {
                    let diff = sam_p.sides - ref_p.sides;
                    if diff.gt(threshold_lower).all() && diff.lt(threshold_upper).all() {
                        let tx = get_transform_matrix(ref_p.stars, sam_p.stars);
                        let proof = sample_objects
                            .iter()
                            .filter_map(|&s_o| {
                                let s_o_tx = (tx * Matrix3x1::from_point(&s_o.to_f64())).to_point().to_f32();
                                self.stars
                                    .iter()
                                    .find(|&r_o| r_o.is_close_to(s_o_tx, threshold))
                                    .map(|&r_o| (r_o, s_o))
                            })
                            .collect::<Vec<_>>();
                        //println!("proofs: {}", proof.len());
                        if proof.len() >= N_PROOF {
                            //println!("found match");
                            let mut tx = Default::default();
                            for w in proof.windows(3) {
                                tx += get_transform_matrix(
                                    [w[0].1, w[1].1, w[2].1],
                                    [w[0].0, w[1].0, w[2].0]);
                            }
                            tx /= (proof.len() - 2) as f64;
                            return Some(tx);
                        }
                    }
                }
            }
        }
        None
    }
}

pub fn extract(path: &str) -> Vec<Point<f32>> {
    let mut objects = sextractor::extract(path);
    // sort by flux, descending
    objects.sort_by(|a,b| b.flux.partial_cmp(&a.flux).unwrap());
    objects
        .into_iter()
        .take(N_OBJECTS)
        .map(|o| Point { x: o.x, y: o.y })
        .collect()
}

/// Returns the matrix that transforms the triangle `src` to `dst`.
fn get_transform_matrix(dst: [Point<f32>; 3], src: [Point<f32>; 3]) -> Matrix3x3<f64> {
    fn poly_to_matrix(points: [Point<f32>; 3]) -> Matrix3x3<f64> {
        Matrix3x3 {
            v11: points[0].x,
            v21: points[0].y,
            v31: 1.0,
            v12: points[1].x,
            v22: points[1].y,
            v32: 1.0,
            v13: points[2].x,
            v23: points[2].y,
            v33: 1.0,
        }.to_f64()
    }
    let res = poly_to_matrix(dst) * poly_to_matrix(src).inverse();
    assert!(!res.has_nan(), "matrix has nan: {:?}", res);
    res
}

#[inline]
fn angle(stars: [Point<f32>; 3]) -> f32 {
    (stars[2] - stars[0]).angle() - (stars[1] - stars[0]).angle()
}

fn polys<'a>(objects: &'a [Point<f32>]) -> impl Iterator<Item=Polygon> + 'a {
    //println!("stars detected: {}", objects.len());
    assert!(objects.len() > 2);

    fn make_poly(window: [&Point<f32>; 3]) -> Polygon {
        let mut stars = [*window[0], *window[1], *window[2]];

        // make sure all triangles are clockwise
        let poly_angle = angle(stars);
        if poly_angle < 0.0 {
            stars.reverse();
        }

        Polygon {
            sides: f32x4::new(
                (stars[1] - stars[0]).length(),
                (stars[2] - stars[1]).length(),
                (stars[0] - stars[2]).length(),
                0.0),
            stars: stars,
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
    use test::Bencher;
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

    #[test]
    fn gen_data() {
        write_stars("test/a.fits", "test/a.stars.json");
        write_stars("test/b.fits", "test/b.stars.json");
    }

    #[test]
    fn test_align() {
        let ref_stars = read_stars("test/a.stars.json");
        let sam_stars = read_stars("test/b.stars.json");
        let r = Reference::from_stars(ref_stars.clone());
        let res = r.align_stars(&sam_stars[..]).unwrap();
        let i = 2;
        assert_eq!(
            (res * Matrix3x1::from_point(&ref_stars[i].to_f64())).to_point().to_f32(),
            sam_stars[i]);
    }

    #[bench]
    fn bench_new(b: &mut Bencher) {
        let ref_stars = read_stars("test/a.stars.json");
        b.iter(|| {
            Reference::from_stars(ref_stars.clone())
        });
    }

    #[bench]
    fn bench_align(b: &mut Bencher) {
        let r = Reference::from_stars(read_stars("test/a.stars.json"));
        let sam_stars = read_stars("test/b.stars.json");
        b.iter(|| {
            r.align_stars(&sam_stars[..]).unwrap()
        });
    }
}
