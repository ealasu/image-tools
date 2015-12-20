use std::cmp::PartialOrd;
use std::collections::BTreeSet;
use itertools::Itertools;
use point::Point;
use types::*;


fn distance(p1: Star, p2: Star) -> f32 {
    let a = p2.x - p1.x;
    let b = p2.y - p1.y;
    ((a * a) + (b * b)).sqrt()
}


#[derive(Copy, Clone, Debug)]
struct Triangle {
    a: Star,
    b: Star,
    c: Star,
    a_to_b: f32,
    b_to_c: f32,
    c_to_a: f32,
}

impl Triangle {
    fn new(a: Star, b: Star, c: Star) -> Triangle {
        Triangle {
            a: a,
            b: b,
            c: c,
            a_to_b: distance(a, b),
            b_to_c: distance(b, c),
            c_to_a: distance(c, a),
        }
    }
}

impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        (self.a == other.a && self.b == other.b && self.c == other.c) ||
        (self.a == other.b && self.b == other.c && self.c == other.a) ||
        (self.a == other.c && self.b == other.a && self.c == other.b)
    }
}

enum Sides {
    AtoB,
    BtoC,
    CtoA,
}


fn max_by<I,F,C>(iter: I, start: C, compare: F) -> I::Item
where I: Iterator, C: PartialOrd, F: Fn(&I::Item) -> C {
    iter.fold((start, None), |(max_c, max_v), v| {
        let c = compare(&v);
        if c > max_c {
            (c, Some(v))
        } else {
            (max_c, max_v)
        }
    }).1.unwrap()
}

fn make_triangles(stars: &[Star]) -> Vec<Triangle> {
    let triangles = stars.iter().map(|&first_star| {
        let second_star = max_by(stars.iter().cloned(), 0.0f32, |&star| {
            distance(star, first_star)
        });
        let third_star = max_by(stars.iter().cloned(), 0.0f32, |&star| {
            distance(star, first_star) + distance(star, second_star)
        });
        Triangle::new(first_star, second_star, third_star)
    }).collect::<Vec<Triangle>>();
    let mut deduped_triangles = Vec::new();
    for &t in triangles.iter() {
        if !deduped_triangles.iter().any(|&i| i == t) {
            deduped_triangles.push(t); 
        }
    }
    deduped_triangles 
}

fn is_close(a: f32, b: f32) -> bool {
    (a - b).abs() < 2.1
}

fn find_triangle(t: Triangle, stars: &[Star]) -> Option<Triangle> {
    let side = stars.iter().combinations().filter_map(|(&a, &b)| {
        let d = distance(a, b);
        if is_close(d, t.a_to_b) {
            Some((a, b, Sides::AtoB))
        } else if is_close(d, t.b_to_c) {
            Some((a, b, Sides::BtoC))
        } else if is_close(d, t.c_to_a) {
            Some((a, b, Sides::CtoA))
        } else {
            None
        }
    }).next();
    side.and_then(|(a, b, side)| {
        let (b_to_c, c_to_a) = match side {
            Sides::AtoB => (t.b_to_c, t.c_to_a),
            Sides::BtoC => (t.c_to_a, t.a_to_b),
            Sides::CtoA => (t.a_to_b, t.b_to_c),
        };
        stars.iter().find(|&&p| {
            is_close(distance(p, a), c_to_a) &&
            is_close(distance(p, b), b_to_c)
        }).map(|&c| {
            Triangle::new(a, b, c)
        })
    })
}

pub fn compute_transform(ref_stars: &Stars, other_stars: &Stars) -> Point<f32> {
    let ts = make_triangles(ref_stars);
    println!("triangles:");
    for t in ts {
        let m = find_triangle(t, other_stars);
        println!("match: {:?}", m);
        //println!("{},{},{},{},{},{}", t.a.x, t.a.y, t.b.x, t.b.y, t.c.x, t.c.y);
    }

    panic!()
}
