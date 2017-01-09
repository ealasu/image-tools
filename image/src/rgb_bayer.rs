use std::ops::*;

#[derive(Copy, Clone, PartialEq, Default)]
pub struct RgbBayer {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub rc: f32,
    pub gc: f32,
    pub bc: f32,
}

impl AddAssign for RgbBayer {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.rc += rhs.rc;
        self.gc += rhs.gc;
        self.bc += rhs.bc;
    }
}

impl DivAssign<f32> for RgbBayer {
    fn div_assign(&mut self, rhs: f32) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
        self.rc /= rhs;
        self.gc /= rhs;
        self.bc /= rhs;
    }
}

impl Mul<f32> for RgbBayer {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        RgbBayer {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            rc: self.rc * rhs,
            gc: self.gc * rhs,
            bc: self.bc * rhs,
        }
    }
}
