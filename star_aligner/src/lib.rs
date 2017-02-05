#![feature(conservative_impl_trait)]
#![feature(test)]

extern crate sextractor;
extern crate geom;
extern crate imagemagick;
extern crate simd;
//extern crate ndarray;
//extern crate ndarray_linalg;
extern crate rulinalg;
#[cfg(test)] extern crate serde_json;
#[cfg(test)] extern crate test;

mod rigid_body;

use std::path::Path;
use std::f64;
use std::iter;
use geom::{Point, Matrix3x3};
use simd::f32x4;

const N_OBJECTS: usize = 400;
const N_PROOF: usize = 250;

#[derive(Debug, Clone)]
pub struct Polygon {
    sides: f32x4,
    stars: [Point<f64>; 3],
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
    stars: Vec<Point<f64>>,
    polys: Vec<Polygon>,
}

impl Reference {
    pub fn from_image<P: AsRef<Path>>(path: P) -> Self {
        Self::from_stars(extract(path))
    }

    pub fn from_stars(stars: Vec<Point<f64>>) -> Self {
        let polys = polys(&stars).collect();
        Reference {
            polys: polys,
            stars: stars,
        }
    }

    pub fn align_image<P: AsRef<Path>>(&self, sample: P) -> Option<Matrix3x3<f64>> {
        self.align_stars(&extract(sample))
    }

    pub fn align_stars(&self, sample_objects: &[Point<f64>]) -> Option<Matrix3x3<f64>> {
        let threshold: f64 = 0.5;

        let threshold_lower = f32x4::splat(-threshold as f32);
        let threshold_upper = f32x4::splat(threshold as f32);
        polys(sample_objects).flat_map(|sam_p| {
            vec![
                sam_p.shift(0),
                sam_p.shift(1),
                sam_p.shift(2),
            ].into_iter()
        }).flat_map(|sam_p| {
            iter::repeat(sam_p).zip(self.polys.iter())
        }).filter(|&(ref sam_p, ref ref_p)| {
            let diff = sam_p.sides - ref_p.sides;
            diff.gt(threshold_lower).all() && diff.lt(threshold_upper).all()
        }).map(|(sam_p, ref_p)| {
            let tx = rigid_body::get_transform_matrix(ref_p.stars, sam_p.stars);
            sample_objects
                .iter()
                .filter_map(|&s_o| {
                    let s_o_tx = (tx * s_o.to_f64()).to_f64();
                    self.stars
                        .iter()
                        .find(|&r_o| r_o.is_close_to(s_o_tx, threshold))
                        .map(|&r_o| (r_o, s_o))
                })
                .collect::<Vec<_>>()
        }).max_by(|a, b| a.len().cmp(&b.len()))
        .and_then(|matching_stars| {
            //println!("proofs: {}", matching_stars.len());
            if matching_stars.len() >= N_PROOF {
                Some(rigid_body::align_simple(&matching_stars))
                //Some(rigid_body::align_all(&matching_stars))
            } else {
                None
            }
        })
    }
}

pub fn extract<P: AsRef<Path>>(path: P) -> Vec<Point<f64>> {
    let image_info = imagemagick::identify(path.as_ref());
    let mut objects = sextractor::extract(path);
    // sort by flux, descending
    objects.sort_by(|a,b| b.flux.partial_cmp(&a.flux).unwrap());
    objects
        .into_iter()
        .take(N_OBJECTS)
        .map(|o| Point { x: o.x as f64, y: image_info.height as f64 - o.y as f64 })
        .collect()
}


#[inline]
fn angle(stars: [Point<f64>; 3]) -> f64 {
    (stars[2] - stars[0]).angle() - (stars[1] - stars[0]).angle()
}

fn polys<'a>(objects: &'a [Point<f64>]) -> impl Iterator<Item=Polygon> + 'a {
    //println!("stars detected: {}", objects.len());
    assert!(objects.len() > 2);

    fn make_poly(window: [&Point<f64>; 3]) -> Polygon {
        let mut stars = [*window[0], *window[1], *window[2]];

        // make sure all triangles are clockwise
        let poly_angle = angle(stars);
        if poly_angle < 0.0 {
            stars.reverse();
        }

        Polygon {
            sides: f32x4::new(
                (stars[1] - stars[0]).length() as f32,
                (stars[2] - stars[1]).length() as f32,
                (stars[0] - stars[2]).length() as f32,
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

    fn read_stars(filename: &str) -> Vec<Point<f64>> {
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
        write_stars("test/c_r.fits", "test/c_r.stars.json");
        write_stars("test/c_s.fits", "test/c_s.stars.json");
    }

    #[test]
    fn test_align() {
        let ref_stars = read_stars("test/a.stars.json");
        let sam_stars = read_stars("test/b.stars.json");
        let i = 0;
        assert_eq!(
            sam_stars[i],
            Point { x: 321.422, y: 2659.7307 });
        //println!("d: {:?}", sam_stars[i] - ref_stars[i]);
        let r = Reference::from_stars(ref_stars.clone());
        let tx = r.align_stars(&sam_stars[..]).unwrap();
        println!("sample: {:?}", sam_stars[i]);
        println!("ref:    {:?}", ref_stars[i]);
        println!("d: {:?}", sam_stars[i] - (tx * ref_stars[i]));
        assert_eq!(
            (tx * ref_stars[i]),
            Point { x: 321.3203286980832, y: 2659.694397022174 });
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
            r.align_stars(&sam_stars[..])
            //r.align_stars(&sam_stars[..]).unwrap()
        });
    }
}
