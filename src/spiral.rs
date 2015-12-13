use point::IPoint;


#[derive(Debug)]
pub struct Spiral {
    r: usize,
    tx: isize,
    ty: isize,
    x: isize,
    y: isize,
}

pub fn spiral() -> Spiral {
    Spiral {
        r: 1,
        tx: 1,
        ty: 0,
        x: 0,
        y: 0,
    }
}

impl Iterator for Spiral {
    type Item = (usize, SpiralPoints);

    fn next(&mut self) -> Option<Self::Item> {
        let ret = Some((self.r, SpiralPoints {
            points_left: self.r,
            tx: self.tx,
            ty: self.ty,
            x: self.x + self.tx,
            y: self.y + self.ty,
        }));

        // move
        self.x += self.tx * self.r as isize;
        self.y += self.ty * self.r as isize;

        // bump side length every other side
        if self.tx == 0 {
            self.r += 1;
        }

        // rotate 90 degrees clockwise
        let tx = -self.ty;
        let ty = self.tx;
        self.tx = tx;
        self.ty = ty;

        ret
    }
}

pub struct SpiralPoints {
    points_left: usize,
    x: isize,
    y: isize,
    tx: isize,
    ty: isize,
}

impl Iterator for SpiralPoints {
    type Item = IPoint;

    fn next(&mut self) -> Option<IPoint> {
        if self.points_left == 0 {
            return None;
        }
        self.points_left -= 1;
        let ret = IPoint {x: self.x, y: self.y};
        self.x += self.tx;
        self.y += self.ty;
        Some(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use point::IPoint;

    #[test]
    fn test() {
        let mut s = spiral();

        let (r, p) = s.next().unwrap();
        assert_eq!(r, 1);
        assert_eq!(p.collect::<Vec<_>>(), vec![
                   IPoint {x: 1, y: 0}]);

        let (r, p) = s.next().unwrap();
        assert_eq!(r, 1);
        assert_eq!(p.collect::<Vec<_>>(), vec![
                   IPoint {x: 1, y: 1}]);

        let (r, p) = s.next().unwrap();
        assert_eq!(r, 2);
        assert_eq!(p.collect::<Vec<_>>(), vec![
                   IPoint {x: 0, y: 1},
                   IPoint {x: -1, y: 1}]);

        let (r, p) = s.next().unwrap();
        assert_eq!(r, 2);
        assert_eq!(p.collect::<Vec<_>>(), vec![
                   IPoint {x: -1, y: 0},
                   IPoint {x: -1, y: -1}]);

        let (r, p) = s.next().unwrap();
        assert_eq!(r, 3);
        assert_eq!(p.collect::<Vec<_>>(), vec![
                   IPoint {x: 0, y: -1},
                   IPoint {x: 1, y: -1},
                   IPoint {x: 2, y: -1}]);

        let (r, p) = s.next().unwrap();
        assert_eq!(r, 3);
        assert_eq!(p.collect::<Vec<_>>(), vec![
                   IPoint {x: 2, y: 0},
                   IPoint {x: 2, y: 1},
                   IPoint {x: 2, y: 2}]);
    }
}
