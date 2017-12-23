use std::mem;
use std::slice;
use num::{Bounded, ToPrimitive, Float, NumCast, Num};
use ndarray::prelude::*;
use ndarray::{Data, Ix};
use ::rgb::Rgb;

pub trait ConvertArray {
    type A;

    fn stretch<B>(&self, src_min: Self::A, src_max: Self::A, dst_min: B, dst_max: B) -> Array2<B>
        where Self::A: Copy + Float + Bounded + ToPrimitive, B: Num + Bounded + NumCast;
    fn stretch_to_bounds<B>(&self) -> Array2<B>
        where Self::A: Copy + Float + Bounded + ToPrimitive, B: Num + Bounded + NumCast;

    fn scale_to_f32(&self) -> Array2<f32>
        where Self::A: Copy + Num + Bounded + ToPrimitive;
    fn scale_to_f64(&self) -> Array2<f64>
        where Self::A: Copy + Num + Bounded + ToPrimitive;
    fn min_max(&self) -> (Self::A, Self::A)
        where Self::A: Copy + Num + Bounded + ToPrimitive + PartialOrd;

    fn to_rgb(&self) -> Array2<Rgb<Self::A>>
        where Self::A: Copy;
    fn as_bytes(&self) -> &[u8];

    fn crop(&self, x: usize, y: usize, width: usize, height: usize) -> Array2<Self::A>
        where Self::A: Copy + Clone;

    fn center_crop(&self, width: usize, height: usize) -> Array2<Self::A>
        where Self::A: Copy + Clone;
}

impl<A, S> ConvertArray for ArrayBase<S, Dim<[Ix; 2]>>
where S: Data<Elem = A>
{
    type A = A;

    fn stretch<B>(&self, src_min: Self::A, src_max: Self::A, dst_min: B, dst_max: B) -> Array2<B>
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

    fn stretch_to_bounds<B>(&self) -> Array2<B>
    where Self::A: Copy + Float + Bounded + ToPrimitive, B: Num + Bounded + NumCast {
        let dst_min = <B as Bounded>::min_value();
        let dst_max = <B as Bounded>::max_value();
        let (src_min, src_max) = self.min_max();
        self.stretch(src_min, src_max, dst_min, dst_max)
    }

    fn scale_to_f32(&self) -> Array2<f32>
    where Self::A: Copy + Num + Bounded + ToPrimitive {
        let src_max = <Self::A as Bounded>::max_value().to_f32().unwrap();
        let dst_max = <f32 as Bounded>::max_value();
        self.mapv(|p| p.to_f32().unwrap_or(dst_max) / src_max)
    }

    fn scale_to_f64(&self) -> Array2<f64>
    where Self::A: Copy + Num + Bounded + ToPrimitive {
        let src_max = <Self::A as Bounded>::max_value().to_f64().unwrap();
        let dst_max = <f64 as Bounded>::max_value();
        self.mapv(|p| p.to_f64().unwrap_or(dst_max) / src_max)
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

    fn to_rgb(&self) -> Array2<Rgb<Self::A>>
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

    fn crop(&self, x: usize, y: usize, width: usize, height: usize) -> Array2<Self::A>
    where Self::A: Copy + Clone {
        assert!(x + width <= self.shape()[1], "x too big: {}", x);
        assert!(y + height <= self.shape()[0]);
        let slice = self.view().into_slice().unwrap();
        let mut pixels = Vec::with_capacity(width * height);
        for y in y..y + width {
            let start = y * self.shape()[1] + x;
            let end = start + width;
            pixels.extend_from_slice(&slice[start..end]);
        }
        Array2::from_shape_vec((height, width), pixels).unwrap()
    }

    fn center_crop(&self, width: usize, height: usize) -> Array2<Self::A>
    where Self::A: Copy + Clone {
        self.crop((self.shape()[1] - width) / 2, (self.shape()[0] - height) / 2, width, height)
    }
}
