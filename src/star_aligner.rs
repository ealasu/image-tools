use itertools::Itertools;
use point::*;
use types::*;
use triangle::Triangle;
use math::*;


const EPSILON: f32 = 0.7;

#[derive(Debug)]
enum Sides {
    AB,
    BC,
    CA,
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


fn find_triangle(t: Triangle, stars: &[Star]) -> Option<Triangle> {
    // TODO: support rotation transforms
    let (a, b, c) = (t.a, t.b, t.c);
    //println!("abc: {:?} {:?} {:?} ", a, b, c);

    let mut matches = stars.iter().combinations().filter_map(|(&ap, &bp)| {
        let d = distance(ap, bp);
        if are_close(d, t.a_to_b, EPSILON) {
            Some((ap, bp, Sides::AB))
        } else if are_close(d, t.b_to_c, EPSILON) {
            Some((ap, bp, Sides::BC))
        } else if are_close(d, t.c_to_a, EPSILON) {
            Some((ap, bp, Sides::CA))
        } else {
            None
        }
    }).filter_map(|(ap, bp, side)| {
        // redefine a,b,c based on the side that was found
        let (a, b, c) = match side {
            Sides::AB => (a, b, c),
            Sides::BC => (b, c, a),
            Sides::CA => (c, a, b),
        };

        // make sure they're oriented right
        let (ap, bp) = if (ap + (b - a)).is_close_to(bp, EPSILON) {
            (ap, bp)
        } else if (bp + (a - b)).is_close_to(ap, EPSILON) {
            (bp, ap)
        } else {
            return None;
        };

        // when we support rotation transforms, do this for each orientation of a'b'
        let cp = c + (ap - a);
        //println!("side: {:?}", side);

        stars.iter().find(|&&star| {
            star.is_close_to(cp, EPSILON)
        }).map(|&cp| {
            let (ap, bp, cp) = match side {
                Sides::AB => (ap, bp, cp),
                Sides::BC => (cp, ap, bp),
                Sides::CA => (bp, cp, ap),
            };
            //println!("a'b'c': {:?} {:?} {:?} ", ap, bp, cp);
            Triangle::new(ap, bp, cp)
        })
    }).collect::<Vec<_>>();
    matches.dedup();
    if matches.len() > 1 {
        println!("too many matches: {}", matches.len());
        for t in matches.iter() {
            println!("{:?}", t);
        }
        return None;
    }
    matches.into_iter().next()
}

pub fn find_matching_triangles(ref_stars: &[Star], other_stars: &[Star]) -> Vec<(Triangle, Triangle)> {
    let ref_triangles = make_triangles(ref_stars);
    ref_triangles.iter().filter_map(|&t| {
        find_triangle(t, other_stars).map(|m| (t, m))
    }).collect()
}

pub fn compute_transform(ref_stars: &Stars, other_stars: &Stars) -> Option<Vector> {
    let matches = find_matching_triangles(ref_stars, other_stars);
    if matches.is_empty() {
        return None;
    }
    println!("matching triangles: {}", matches.len());
    //for &(t, m) in matches.iter() {
        //println!("match: {}", distance(t.a, m.a));
        //println!("t: {:?}", t);
        //println!("m: {:?}", m);
    //}
    //println!("{},{},{},{},{},{}", t.a.x, t.a.y, t.b.x, t.b.y, t.c.x, t.c.y);
    let transforms = matches.into_iter().map(|(r, m)| {
        // average of all three transforms
        ((r.a - m.a) + (r.b - m.b) + (r.c - m.c)) / 3.0
        //vec![(r.a - m.a), (r.b - m.b), (r.c - m.c)]
    }).collect::<Vec<Vector>>();
    let avg = transforms.iter().fold(Vector {x: 0.0, y: 0.0}, |acc, &i| acc + i) / transforms.len() as f32;
    Some(avg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::*;
    use point::*;
    use triangle::*;

    // same
    #[test]
    fn test_1() {
        let stars_1 = vec![Star {x: 0.0, y: 0.0}, Star {x: 1.0, y: 0.0}, Star {x: 2.0, y: 2.0}];
        let stars_2 = vec![Star {x: 0.0, y: 0.0}, Star {x: 1.0, y: 0.0}, Star {x: 2.0, y: 2.0}];
        let matches = find_matching_triangles(&stars_1, &stars_2);
        let t = Triangle::new(Star {x: 0.0, y: 0.0}, Star {x: 1.0, y: 0.0}, Star {x: 2.0, y: 2.0});
        assert_eq!(matches, vec![(t, t)]);
    }

    // flipped
    #[test]
    fn test_2() {
        let stars_1 = vec![Star {x: 0.0, y: 0.0}, Star {x: 1.0, y: 0.0}, Star {x: 2.0, y: 2.0}];
        let stars_2 = vec![Star {x: 0.0, y: 0.0}, Star {x: 0.0, y: 1.0}, Star {x: 2.0, y: 2.0}];
        let matches = find_matching_triangles(&stars_1, &stars_2);
        assert_eq!(matches, vec![]);
    }

    // shifted right
    #[test]
    fn test_3() {
        let t1 = Triangle::new(Star {x: 0.0, y: 0.0}, Star {x: 1.0, y: 0.0}, Star {x: 2.0, y: 2.0});
        let t2 = Triangle::new(Star {x: 2.0, y: 0.0}, Star {x: 3.0, y: 0.0}, Star {x: 4.0, y: 2.0});
        let stars_1 = vec![t1.a, t1.b, t1.c];
        let stars_2 = vec![t2.a, t2.b, t2.c];
        let matches = find_matching_triangles(&stars_1, &stars_2);
        assert_eq!(matches, vec![(t1, t2)]);
    }

    // shifted up
    #[test]
    fn test_4() {
        let t1 = Triangle::new(Star {x: 0.0, y: 0.0}, Star {x: 1.0, y: 0.0}, Star {x: 2.0, y: 2.0});
        let t2 = Triangle::new(Star {x: 0.0, y: 2.0}, Star {x: 1.0, y: 2.0}, Star {x: 2.0, y: 4.0});
        let stars_1 = vec![t1.a, t1.b, t1.c];
        let stars_2 = vec![t2.a, t2.b, t2.c];
        let matches = find_matching_triangles(&stars_1, &stars_2);
        assert_eq!(matches, vec![(t1, t2)]);
    }

    // same
    #[test]
    fn test_5() {
        let stars_1 = vec![Star {x: 0.0, y: 0.0}, Star {x: 1.0, y: 0.0}, Star {x: 2.0, y: 2.0}];
        let stars_2 = vec![Star {x: 1.0, y: 0.0}, Star {x: 2.0, y: 2.0}, Star {x: 0.0, y: 0.0}];
        let matches = find_matching_triangles(&stars_1, &stars_2);
        let t = Triangle::new(Star {x: 0.0, y: 0.0}, Star {x: 1.0, y: 0.0}, Star {x: 2.0, y: 2.0});
        assert_eq!(matches, vec![(t, t)]);
    }

    #[test]
    fn test_6() {
        let t1 = Triangle::new(Star {x: 1.0, y: 1.0}, Star {x: 2.0, y: 1.0}, Star {x: 3.0, y: 3.0});
        let t2 = Triangle::new(Star {x: 3.0, y: 0.0}, Star {x: 4.0, y: 0.0}, Star {x: 5.0, y: 2.0});
        let stars_1 = vec![t1.a, t1.b, t1.c];
        let stars_2 = vec![t2.a, t2.b, t2.c];
        let matches = find_matching_triangles(&stars_1, &stars_2);
        assert_eq!(matches, vec![(t1, t2)]);
    }

    #[test]
    fn test_7() {
        let t1 = Triangle::new(Point { x: 4134.224, y: 1553.7593 }, Point { x: 1289.2603, y: 1295.592 }, Point { x: 1780.352, y: 2829.1196 });
        let t2 = Triangle::new(Point { x: 4080.8955, y: 1563.5502 }, Point { x: 1236.1643, y: 1305.9685 }, Point { x: 1727.9224, y: 2839.7646 });
        let stars_1 = vec![t1.a, t1.b, t1.c];
        let stars_2 = vec![t2.a, t2.b, t2.c];
        let matches = find_matching_triangles(&stars_1, &stars_2);
        assert_eq!(matches, vec![(t1, t2)]);
    }

    #[test]
    fn test_8() {
        let t1 = Triangle { a: Point { x: 1744.9852, y: 2597.812 }, b: Point { x: 3941.397, y: 835.62366 }, c: Point { x: 1289.2603, y: 1295.592 }, a_to_b: 2815.9424, b_to_c: 2691.728, c_to_a: 1379.6602 };
        let t2 = Triangle { a: Point { x: 1236.1643, y: 1305.9685 }, b: Point { x: 1692.487, y: 2608.3726 }, c: Point { x: 3888.0122, y: 845.60425 }, a_to_b: 1380.0315, b_to_c: 2815.614, c_to_a: 2691.5112 };
        let stars_1 = vec![t1.a, t1.b, t1.c];
        let stars_2 = vec![t2.a, t2.b, t2.c];
        let matches = find_matching_triangles(&stars_1, &stars_2);
        assert_eq!(matches, vec![(t1, t2)]);
    }
}
