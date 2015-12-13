use point::IPoint;

pub fn spiral() -> Spiral {
    Spiral {
        r: 1,
        tx: 1,
        ty: 0,
        x: 0,
        y: 0,
    }
}

pub struct Spiral {
    r: usize,
    tx: isize,
    ty: isize,
    x: isize,
    y: isize,
}

impl Iterator for Spiral {
    type Item = (usize, SpiralPoints);

    fn next(&mut self) -> Option<(usize, SpiralPoints)> {
        let points = SpiralPoints {
            points_left: self.r,
            tx: self.tx,
            ty: self.ty,
            x: self.x,
            y: self.y,
        };

        // rotate 90 degrees clockwise
        let tx = -self.ty;
        let ty = self.tx;
        self.tx = tx;
        self.ty = ty;

        if ty == 0 {
            self.r += 1;
        }

        Some((self.r, points))
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
        let ret = IPoint::new(self.x, self.y);
        self.x += self.tx;
        self.y += self.ty;
        Some(ret)
    }
}
