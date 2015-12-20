use itertools::Itertools;
use point::*;
use types::*;
use triangle::Triangle;
use math::*;


const EPSILON: f32 = 0.9;

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
    let mut matches = stars.iter().combinations().filter_map(|(&a, &b)| {
        let d = distance(a, b);
        if are_close(d, t.a_to_b, EPSILON) {
            Some((a, b, Sides::AB))
        } else if are_close(d, t.b_to_c, EPSILON) {
            Some((a, b, Sides::BC))
        } else if are_close(d, t.c_to_a, EPSILON) {
            Some((a, b, Sides::CA))
        } else {
            None
        }
    }).filter_map(|(a, b, side)| {
        let (c_to_a, b_to_c) = match side {
            Sides::AB => (t.c_to_a, t.b_to_c),
            Sides::BC => (t.a_to_b, t.c_to_a),
            Sides::CA => (t.b_to_c, t.a_to_b),
        };
        stars.iter().find(|&&c| {
            let ca = distance(c, a);
            let cb = distance(c, b);
            (are_close(ca, c_to_a, EPSILON) && are_close(cb, b_to_c, EPSILON)) ||
            (are_close(cb, c_to_a, EPSILON) && are_close(ca, b_to_c, EPSILON))
        }).map(|&c| {
            //let m_b_to_c = distance(b, c);
            //let m_c_to_a = distance(c, a);
            //let (a, b) = if m_b_to_c > m_c_to_a && b_to_c < c_to_a {
                //(b, a)
            //} else {
                //(a, b)
            //};
            let (a, b, c) = match side {
                Sides::AB => (a, b, c),
                Sides::BC => (b, c, a),
                Sides::CA => (c, a, b),
            };
            Triangle::new(a, b, c)
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

    #[test]
    fn test_1() {
        let stars_1 = vec![Star {x: 0.0, y: 0.0}, Star {x: 1.0, y: 0.0}, Star {x: 2.0, y: 2.0}];
        let stars_2 = vec![Star {x: 0.0, y: 0.0}, Star {x: 1.0, y: 0.0}, Star {x: 2.0, y: 2.0}];
        let matches = find_matching_triangles(&stars_1, &stars_2);
        let t = Triangle::new(Star {x: 0.0, y: 0.0}, Star {x: 1.0, y: 0.0}, Star {x: 2.0, y: 2.0});
        assert_eq!(matches, vec![(t, t)]);
    }

    #[test]
    fn test_2() {
        let stars_1 = vec![Star {x: 0.0, y: 0.0}, Star {x: 1.0, y: 0.0}, Star {x: 2.0, y: 2.0}];
        let stars_2 = vec![Star {x: 0.0, y: 0.0}, Star {x: 0.0, y: 1.0}, Star {x: 2.0, y: 2.0}];
        let matches = find_matching_triangles(&stars_1, &stars_2);
        assert_eq!(matches, vec![]);
    }
}
