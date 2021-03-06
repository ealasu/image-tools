use std::ops::*;
use rand::{Rng, Rand};
use util::*;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Rgb<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl Rgb<f32> {
    pub fn truncate(&self, lower: f32, upper: f32) -> Self {
        Rgb {
            r: max(lower, min(upper, self.r)),
            g: max(lower, min(upper, self.g)),
            b: max(lower, min(upper, self.b)),
        }
    }
}

impl<T: Rand> Rand for Rgb<T> {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        Rgb {
            r: rng.gen(),
            g: rng.gen(),
            b: rng.gen(),
        }
    }
}

impl<T: Sub<Output=T>> Sub for Rgb<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Rgb {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl<T: AddAssign> AddAssign for Rgb<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl<T: Copy + DivAssign> DivAssign<T> for Rgb<T> {
    fn div_assign(&mut self, rhs: T) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
    }
}

impl<T: Mul<Output=T>> Mul for Rgb<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Rgb {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl<T: Copy + Mul<T,Output=T>> Mul<T> for Rgb<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Rgb {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}
