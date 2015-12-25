use types::*;
use math::*;


#[derive(Copy, Clone, Debug)]
pub struct Triangle {
    pub a: Star,
    pub b: Star,
    pub c: Star,
    pub a_to_b: f32,
    pub b_to_c: f32,
    pub c_to_a: f32,
}

impl Triangle {
    pub fn new(a: Star, b: Star, c: Star) -> Triangle {
        // make a -> b -> c always be clockwise
        let v_ab = b - a; // vector to get from a to b
        let v_ac = c - a; // vector to get from a to c
        let (b, c) = if v_ab.cross_product(v_ac) < 0.0 {
            (b, c)
        } else {
            (c, b)
        };
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

#[cfg(test)]
mod tests {
    use super::*;
    use types::*;

    #[test]
    fn test_order_1() {
        let t = Triangle::new(Star {x: 0.0, y: 0.0}, Star {x: 0.0, y: 1.0}, Star {x: 1.0, y: 0.0});
        assert_eq!(t.a, Star {x: 0.0, y: 0.0});
        assert_eq!(t.b, Star {x: 0.0, y: 1.0});
        assert_eq!(t.c, Star {x: 1.0, y: 0.0});
    }

    #[test]
    fn test_order_2() {
        let t = Triangle::new(Star {x: 0.0, y: 0.0}, Star {x: 1.0, y: 0.0}, Star {x: 0.0, y: 1.0});
        assert_eq!(t.a, Star {x: 0.0, y: 0.0});
        assert_eq!(t.b, Star {x: 0.0, y: 1.0});
        assert_eq!(t.c, Star {x: 1.0, y: 0.0});
    }

    #[test]
    fn test_eq_1() {
        let a = Triangle::new(Star {x: 0.0, y: 0.0}, Star {x: 0.0, y: 1.0}, Star {x: 1.0, y: 0.0});
        let b = Triangle::new(Star {x: 0.0, y: 0.0}, Star {x: 0.0, y: 1.0}, Star {x: 1.0, y: 0.0});
        assert_eq!(a, b);
    }

    #[test]
    fn test_eq_2() {
        let a = Triangle::new(Star {x: 0.0, y: 0.0}, Star {x: 0.0, y: 1.0}, Star {x: 1.0, y: 0.0});
        let b = Triangle::new(Star {x: 0.0, y: 1.0}, Star {x: 1.0, y: 0.0}, Star {x: 0.0, y: 0.0});
        assert_eq!(a, b);
    }

    #[test]
    fn test_eq_3() {
        let a = Triangle::new(Star {x: 0.0, y: 0.0}, Star {x: 0.0, y: 1.0}, Star {x: 1.0, y: 0.0});
        let b = Triangle::new(Star {x: 0.0, y: 0.0}, Star {x: 1.0, y: 0.0}, Star {x: 0.0, y: 1.0});
        assert_eq!(a, b);
    }
}
