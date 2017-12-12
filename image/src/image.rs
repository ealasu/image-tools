use std::default::Default;
use std::fmt;
use std::iter::repeat;
use std::ops::*;
use std::convert::{AsRef, AsMut};
use rand::{self, Rng, Rand};
use num::{Bounded, ToPrimitive, Float};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ImageDimensions {
    pub width: usize,
    pub height: usize,
    pub pitch: usize,
}

pub struct OwnedImage<P> {
    pub dimensions: ImageDimensions,
    pub pixels: Vec<P>,
}

pub struct ImageSlice<'a, P: 'a> {
    pub dimensions: ImageDimensions,
    pub pixels: &'a [P],
}

pub struct ImageSliceMut<'a, P: 'a> {
    pub dimensions: ImageDimensions,
    pub pixels: &'a mut [P],
}


impl<'a, P: 'a> AsRef<ImageSlice<'a, P>> for OwnedImage<P> {
    #[inline(always)]
    fn as_ref(&self) -> &ImageSlice<'a, P> {
        unsafe {
            &*(self as *const OwnedImage<P> as *const ImageSlice<P>)
        }
    }
}

impl<'a, P: 'a> AsMut<ImageSliceMut<'a, P>> for OwnedImage<P> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut ImageSliceMut<'a, P> {
        unsafe {
            &mut *(self as *mut OwnedImage<P> as *mut ImageSliceMut<P>)
        }
    }
}

impl<'a, P: 'a> AsRef<ImageSlice<'a, P>> for ImageSlice<'a, P> {
    #[inline(always)]
    fn as_ref(&self) -> &ImageSlice<'a, P> {
        self
    }
}

impl<'a, P: 'a> AsRef<ImageSlice<'a, P>> for ImageSliceMut<'a, P> {
    #[inline(always)]
    fn as_ref(&self) -> &ImageSlice<'a, P> {
        unsafe {
            &*(self as *const ImageSliceMut<P> as *const ImageSlice<P>)
        }
    }
}

impl<'a, P: 'a> AsMut<ImageSliceMut<'a, P>> for ImageSliceMut<'a, P> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut ImageSliceMut<'a, P> {
        self
    }
}

macro check_x($x:expr, $self:expr) {
    let dim = $self.dimensions();
    debug_assert!($x < dim.width, "{} ({}) >= width ({})", stringify!($x), $x, dim.width);
}

macro check_y($y:expr, $self:expr) {
    let dim = $self.dimensions();
    debug_assert!($y < dim.height, "{} ({}) >= height ({})", stringify!($y), $y, dim.height);
}

impl<Pixel> Image for OwnedImage<Pixel> {
    type Pixel = Pixel;

    fn dimensions(&self) -> ImageDimensions {
        self.dimensions
    }

    fn pixels(&self) -> &[Self::Pixel] {
        &self.pixels
    }
}
impl<'a, Pixel: 'a> Image for ImageSlice<'a, Pixel> {
    type Pixel = Pixel;

    fn dimensions(&self) -> ImageDimensions {
        self.dimensions
    }

    fn pixels(&self) -> &[Self::Pixel] {
        self.pixels
    }
}
impl<'a, Pixel: 'a> Image for ImageSliceMut<'a, Pixel> {
    type Pixel = Pixel;

    fn dimensions(&self) -> ImageDimensions {
        self.dimensions
    }

    fn pixels(&self) -> &[Self::Pixel] {
        self.pixels
    }
}
impl<'a, Pixel: 'a> ImageMut for ImageSliceMut<'a, Pixel> {
    fn pixels_mut(&mut self) -> &mut [Self::Pixel] {
        self.pixels
    }
}

pub trait ImageMut: Image {
    fn pixels_mut(&mut self) -> &mut [Self::Pixel];

    #[inline(always)]
    fn pixel_at_mut(&mut self, x: usize, y: usize) -> &mut Self::Pixel {
        check_x!(x, self);
        check_y!(y, self);
        let pitch = self.dimensions().pitch;
        &mut self.pixels_mut()[x + y * pitch]
    }
}

pub trait Image {
    type Pixel;

    fn dimensions(&self) -> ImageDimensions;
    fn pixels(&self) -> &[Self::Pixel];

    #[inline(always)]
    fn clone_map<'a, F, R>(&'a self, f: F) -> OwnedImage<R>
    where F: FnMut(&'a Self::Pixel) -> R {
        OwnedImage {
            dimensions: self.dimensions(),
            pixels: self.pixels().iter().map(f).collect(),
        }
    }

    #[inline(always)]
    fn pixel_at(&self, x: usize, y: usize) -> &Self::Pixel {
        check_x!(x, self);
        check_y!(y, self);
        &self.pixels()[x + y * self.dimensions().pitch]
    }

    #[inline(always)]
    fn row(&self, y: usize) -> &[Self::Pixel] {
        check_y!(y, self);
        let i = y * self.dimensions().pitch;
        &self.pixels()[i..i + self.dimensions().width]
    }

    #[inline(always)]
    fn row_slice(&self, y: usize, left: usize, right: usize) -> &[Self::Pixel] {
        check_y!(y, self);
        check_x!(left, self);
        check_x!(left + right, self);
        let i = y * self.dimensions().pitch;
        &self.pixels()[i + left..i + self.dimensions().width - right]
    }

    fn min(&self) -> &Self::Pixel
    where Self::Pixel: PartialOrd {
        self.pixels().iter().min_by(|a,b| a.partial_cmp(b).unwrap()).unwrap()
    }

    fn max(&self) -> &Self::Pixel
    where Self::Pixel: PartialOrd {
        self.pixels().iter().max_by(|a,b| a.partial_cmp(b).unwrap()).unwrap()
    }

    fn crop<'a>(&'a self, x: usize, y: usize, width: usize, height: usize) -> ImageSlice<'a, Self::Pixel> {
        assert!(x + width <= self.dimensions().width, "x too big: {}", x);
        assert!(y + height <= self.dimensions().height);
        ImageSlice {
            dimensions: ImageDimensions {
                width,
                height,
                pitch: self.dimensions().pitch,
            },
            pixels: &self.pixels()[x + y * self.dimensions().pitch..],
        }
    }

    fn center_crop<'a>(&'a self, width: usize, height: usize) -> ImageSlice<'a, Self::Pixel> {
        self.crop((self.dimensions().width - width) / 2, (self.dimensions().height - height) / 2, width, height)
    }
}

//impl<'a, Pixel: 'a> ImageSliceMut<'a, Pixel> {
//}

// TODO when https://github.com/rust-lang/rust/issues/44265 is fixed
//impl<'a, P> Deref for &'a Image<P>
//where P: 'a {
    //type Target = ImageSlice<'a, P>;

    //fn deref(&'a self) -> &ImageSlice<'a, P> {
        //&*(self as *const OwnedImage<P> as *const ImageSlice<P>)
    //}
//}


impl<'a, P> fmt::Debug for ImageSlice<'a, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[image {:?}]", self.dimensions())
    }
}

impl<P> fmt::Debug for OwnedImage<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<'a, P> fmt::Debug for ImageSliceMut<'a, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<'a, P: AddAssign + Copy> Add<ImageSlice<'a, P>> for OwnedImage<P> {
    type Output = Self;
    fn add(mut self, rhs: ImageSlice<'a, P>) -> Self {
        self.as_mut().add_assign(rhs);
        self
    }
}

impl<'a, P: AddAssign + Copy> AddAssign<ImageSlice<'a, P>> for ImageSliceMut<'a, P> {
    fn add_assign(&mut self, rhs: ImageSlice<'a, P>) {
        assert_eq!(self.dimensions().width, rhs.dimensions().width);
        assert_eq!(self.dimensions().height, rhs.dimensions().height);
        for (l, &r) in self.pixels_mut().iter_mut().zip(rhs.pixels().iter()) {
            l.add_assign(r);
        }
    }
}

impl<P: DivAssign + Copy> Div<P> for OwnedImage<P> {
    type Output = Self;
    fn div(mut self, rhs: P) -> Self {
        self.as_mut().div_assign(rhs);
        self
    }
}

impl<'a, P: DivAssign + Copy> DivAssign<P> for ImageSliceMut<'a, P> {
    fn div_assign(&mut self, rhs: P) {
        for p in self.pixels_mut().iter_mut() {
            *p /= rhs;
        }
    }
}

impl<'a, P: DivAssign + Copy> Div<ImageSlice<'a, P>> for OwnedImage<P> {
    type Output = Self;
    fn div(mut self, rhs: ImageSlice<'a, P>) -> Self {
        self.as_mut().div_assign(rhs);
        self
    }
}

impl<'a, P: DivAssign + Copy> DivAssign<ImageSlice<'a, P>> for ImageSliceMut<'a, P> {
    fn div_assign(&mut self, rhs: ImageSlice<'a, P>) {
        for (left, right) in self.pixels_mut().iter_mut().zip(rhs.pixels().iter()) {
            *left /= *right;
        }
    }
}

impl<P: Copy + Clone + Default> OwnedImage<P> {
    pub fn new(width: usize, height: usize) -> Self {
        let mut pixels = Vec::with_capacity(width * height);
        let zero: P = Default::default();
        pixels.extend(repeat(zero).take(width * height));
        OwnedImage {
            dimensions: ImageDimensions {
                width: width,
                height: height,
                pitch: width,
            },
            pixels: pixels,
        }
    }
}

impl<P: Rand> OwnedImage<P> {
    pub fn random(width: usize, height: usize) -> Self {
        OwnedImage {
            dimensions: ImageDimensions {
                width: width,
                height: height,
                pitch: width,
            },
            pixels: rand::thread_rng().gen_iter().take(width * height).collect()
        }
    }
}

pub trait ScaleCast {
    fn scale_to_f32(&self) -> OwnedImage<f32>;
    fn scale_to_f64(&self) -> OwnedImage<f64>;
}

impl<T, Pixel> ScaleCast for T
where T: Image<Pixel=Pixel>,
      Pixel: Bounded + ToPrimitive + Copy {
    fn scale_to_f32(&self) -> OwnedImage<f32> {
        let src_max = <Pixel as Bounded>::max_value().to_f32().unwrap();
        let dst_max = <f32 as Bounded>::max_value();
        self.clone_map(|&p| p.to_f32().unwrap_or(dst_max) / src_max)
    }

    fn scale_to_f64(&self) -> OwnedImage<f64> {
        let src_max = <Pixel as Bounded>::max_value().to_f64().unwrap();
        let dst_max = <f64 as Bounded>::max_value();
        self.clone_map(|&p| p.to_f64().unwrap_or(dst_max) / src_max)
    }
}

pub trait FloatCast {
    fn to_f32(&self) -> OwnedImage<f32>;
    fn to_f64(&self) -> OwnedImage<f64>;
}

impl<T, Pixel> FloatCast for T
where T: Image<Pixel=Pixel>,
      Pixel: Bounded + Float {
    fn to_f32(&self) -> OwnedImage<f32> {
        let dst_max = <f32 as Bounded>::max_value();
        self.clone_map(|&p| p.to_f32().unwrap_or(dst_max))
    }

    fn to_f64(&self) -> OwnedImage<f64> {
        let dst_max = <f64 as Bounded>::max_value();
        self.clone_map(|&p| p.to_f64().unwrap_or(dst_max))
    }
}

macro impl_cast($name:ident, $from:ty, $to:ty) {
    impl<'a> ImageSlice<'a, $from> {
        pub fn $name(&self) -> OwnedImage<$to> {
            self.clone_map(|&p| p as $to)
        }
    }
}
impl_cast!(to_f64, f32, f64);
impl_cast!(to_f32, f64, f32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_ref() {
        let dimensions = ImageDimensions {
            width: 1,
            height: 2,
            pitch: 3,
        };
        let pixels = vec![4,5,6];
        let mut image = OwnedImage {
            dimensions,
            pixels: pixels.clone(),
        };
        macro test_ref($r:expr) {
            assert_eq!(dimensions, $r.dimensions());
            assert_eq!(&pixels[..], &$r.pixels()[..]);
        }
        test_ref!(image.as_ref());
        test_ref!(image.as_ref().as_ref());
        test_ref!(image.as_mut());
        test_ref!(image.as_mut().as_ref());
        test_ref!(image.as_mut().as_mut());
    }
}
