pub fn cross_range(n: usize) -> CrossRange {
    CrossRange {
        n: n,
        a: (n as isize) + 1,
        b: -(n as isize),
        state: State::A,
    }
}

enum State {
    A,
    B
}

pub struct CrossRange {
    n: usize,
    a: isize,
    b: isize,
    state: State,
}

impl Iterator for CrossRange {
    type Item = (isize, isize);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            State::A => {
                if self.a == -(self.n as isize) {
                    return None;
                }
                self.a -= 1;
                self.state = State::B;
            }
            State::B => {
                if self.b == (self.n as isize) {
                    return None;
                }
                self.b += 1;
                self.state = State::A;
            }
        };
        Some((self.a, self.b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test() {
        let v: Vec<_> = cross_range(3).collect();
        let expected = [
            (3, -3),
            (3, -2),
            (2, -2),
            (2, -1),
            (1, -1),
            (1, 0),
            (0, 0),
            (0, 1),
            (-1, 1),
            (-1, 2),
            (-2, 2),
            (-2, 3),
            (-3, 3),
        ];
        assert_eq!(&v[..], &expected[..]);
    }

    #[test]
    fn test_one() {
        let v: Vec<_> = cross_range(1).collect();
        let expected = [
            (1, -1),
            (1, 0),
            (0, 0),
            (0, 1),
            (-1, 1),
        ];
        assert_eq!(&v[..], &expected[..]);
    }

    #[test]
    fn test_zero() {
        let v: Vec<_> = cross_range(0).collect();
        let expected = [(0, 0)];
        assert_eq!(&v[..], &expected[..]);
    }
}
