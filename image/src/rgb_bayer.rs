use std::ops::*;
use num::Float;

#[derive(Copy, Clone, PartialEq, Default)]
pub struct RgbBayer<P> {
    pub r: P,
    pub g: P,
    pub b: P,
    pub rc: P,
    pub gc: P,
    pub bc: P,
}

impl<P: Float> AddAssign for RgbBayer<P> {
    fn add_assign(&mut self, rhs: Self) {
        self.r  = self.r  + rhs.r;
        self.g  = self.g  + rhs.g;
        self.b  = self.b  + rhs.b;
        self.rc = self.rc + rhs.rc;
        self.gc = self.gc + rhs.gc;
        self.bc = self.bc + rhs.bc;
    }
}

impl<P: Float> DivAssign<P> for RgbBayer<P> {
    fn div_assign(&mut self, rhs: P) {
        self.r  = self.r / rhs;
        self.g  = self.g / rhs;
        self.b  = self.b / rhs;
        self.rc = self.rc / rhs;
        self.gc = self.gc / rhs;
        self.bc = self.bc / rhs;
    }
}

impl<P: Float> Mul<P> for RgbBayer<P> {
    type Output = Self;
    fn mul(self, rhs: P) -> Self::Output {
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
