use std::mem;
use std::slice;
use num::{Bounded, ToPrimitive, Float, NumCast, Num};
use ndarray::prelude::*;
use ndarray::{Data};
use ::rgb::Rgb;

pub trait ConvertArray {
    type A;
    type D: Dimension;

    fn stretch<B>(&self, src_min: Self::A, src_max: Self::A, dst_min: B, dst_max: B) -> Array<B, Self::D>
        where Self::A: Copy + Float + Bounded + ToPrimitive, B: Num + Bounded + NumCast;
    fn stretch_to_bounds<B>(&self) -> Array<B, Self::D>
        where Self::A: Copy + Float + Bounded + ToPrimitive, B: Num + Bounded + NumCast;

    fn scale_to_f32(&self) -> Array<f32, Self::D>
        where Self::A: Copy + Num + Bounded + ToPrimitive;
    fn min_max(&self) -> (Self::A, Self::A)
        where Self::A: Copy + Num + Bounded + ToPrimitive + PartialOrd;

    fn to_rgb(&self) -> Array<Rgb<Self::A>, Self::D>
        where Self::A: Copy;
    fn as_bytes(&self) -> &[u8];
}

impl<A, S, D> ConvertArray for ArrayBase<S, D>
where S: Data<Elem = A>,
      D: Dimension
{
    type A = A;
    type D = D;

    fn stretch<B>(&self, src_min: Self::A, src_max: Self::A, dst_min: B, dst_max: B) -> Array<B, Self::D>
    where Self::A: Copy + Float + Bounded + ToPrimitive, B: Num + Bounded + NumCast {
        let dst_d = <Self::A as NumCast>::from(dst_max - dst_min).unwrap();
        let src_d = src_max - src_min;
        self.mapv(|mut p| {
            if p < src_min {
                p = src_min;
            }
            if p > src_max {
                p = src_max;
            }
            <B as NumCast>::from((p - src_min) * dst_d / src_d).unwrap()
        })
    }

    fn stretch_to_bounds<B>(&self) -> Array<B, Self::D>
    where Self::A: Copy + Float + Bounded + ToPrimitive, B: Num + Bounded + NumCast {
        let dst_min = <B as Bounded>::min_value();
        let dst_max = <B as Bounded>::max_value();
        let (src_min, src_max) = self.min_max();
        self.stretch(src_min, src_max, dst_min, dst_max)
    }

    fn scale_to_f32(&self) -> Array<f32, Self::D>
    where Self::A: Copy + Num + Bounded + ToPrimitive {
        let src_max = <Self::A as Bounded>::max_value().to_f32().unwrap();
        let dst_max = <f32 as Bounded>::max_value();
        self.mapv(|p| p.to_f32().unwrap_or(dst_max) / src_max)
    }

    fn min_max(&self) -> (Self::A, Self::A)
    where Self::A: Copy + Num + Bounded + ToPrimitive + PartialOrd {
        let mut min = <Self::A as Bounded>::max_value();
        let mut max = <Self::A as Bounded>::min_value();
        for &p in self.iter() {
            if p < min {
                min = p;
            }
            if p > max {
                max = p;
            }
        }
        (min, max)
    }

    fn to_rgb(&self) -> Array<Rgb<Self::A>, Self::D>
    where Self::A: Copy {
        self.mapv(|p| Rgb { r: p, g: p, b: p })
    }

    #[inline]
    fn as_bytes(&self) -> &[u8] {
        let s = self.view().into_slice().unwrap();
        unsafe {
            slice::from_raw_parts(
                s.as_ptr() as *const u8,
                s.len() * mem::size_of::<Self::A>())
        }
    }
}
