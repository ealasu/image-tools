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
        //println!("{:?}", side);
        /*
         steps (to find the possible locations of c'):
         find vector ac (c - a)
         for each orientation of a'b' (a'b' or b'a'):
             find the matrix m_ab_to_apbp that transforms ab to a'b' (rotation + translation, maybe ignore rotation for now)
             multiply ac by m_ab_to_apbp to get c'

         then all you have to do is find a star that's close to c'
         */

        // redefine a,b,c based on the side that was found
        let (a, b, c) = match side {
            Sides::AB => (t.a, t.b, t.c),
            Sides::BC => (t.b, t.c, t.a),
            Sides::CA => (t.c, t.a, t.b),
        };

        // make sure they're oriented right
        let (ap, bp) = if (ap + (b - a)).is_close_to(bp, EPSILON) {
            (ap, bp)
        } else if (bp + (a - b)).is_close_to(ap, EPSILON) {
            (bp, ap)
        } else {
            return None;
        };

        // TODO: when we support rotation transforms, do this for each orientation of a'b'
        let cp = c + (ap - a);
        //c - a + (ap - a)
        //println!("abc: {:?} {:?} {:?} ", a, b, c);
        //println!("a'b'c': {:?} {:?} {:?} ", ap, bp, cp);

        stars.iter().find(|&&star| {
            star.is_close_to(cp, EPSILON)
        }).map(|&cp| {
            let (ap, bp, cp) = match side {
                Sides::AB => (ap, bp, cp),
                Sides::BC => (bp, cp, ap),
                Sides::CA => (cp, ap, bp),
            };
            Triangle::new(ap, bp, cp)
        })
    }).collect::<Vec<_>>();
    matches.dedup();
    //println!("tri matches: {}", matches.len());
    // TODO: not needed after star filtering is impld.
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

pub fn compute_transform(ref_stars: &Stars, other_stars: &Stars) -> Vector {
    let matches = find_matching_triangles(ref_stars, other_stars);
    println!("matching triangles: {}", matches.len());
    for &(t, m) in matches.iter() {
        println!("match: {}", distance(t.a, m.a));
        println!("t: {:?}", t);
        println!("m: {:?}", m);
    }
    //println!("{},{},{},{},{},{}", t.a.x, t.a.y, t.b.x, t.b.y, t.c.x, t.c.y);

    Vector {x: 0.0, y: 0.0}
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
}
