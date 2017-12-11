use std::default::Default;
use std::fmt;
use std::iter::repeat;
use std::ops::*;
use std::slice;
//use std::ops::Deref;
use std::convert::{AsRef, AsMut};
use rand::{self, Rng, Rand};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ImageDimensions {
    pub width: usize,
    pub height: usize,
    pub pitch: usize,
}

macro check_x($x:expr, $self:expr) {
    let dim = $self.dimensions;
    debug_assert!($x < dim.width, "{} ({}) >= width ({})", stringify!($x), $x, dim.width);
}

macro check_y($y:expr, $self:expr) {
    let dim = $self.dimensions;
    debug_assert!($y < dim.height, "{} ({}) >= height ({})", stringify!($y), $y, dim.height);
}


pub struct Image<P> {
    dimensions: ImageDimensions,
    pixels: Vec<P>,
}

pub struct ImageSlice<'a, P: 'a> {
    dimensions: ImageDimensions,
    pixels: &'a [P],
}

pub struct ImageSliceMut<'a, P: 'a> {
    dimensions: ImageDimensions,
    pixels: &'a mut [P],
}


impl<'a, P: 'a> AsRef<ImageSlice<'a, P>> for Image<P> {
    #[inline(always)]
    fn as_ref(&self) -> &ImageSlice<'a, P> {
        unsafe {
            &*(self as *const Image<P> as *const ImageSlice<P>)
        }
    }
}

impl<'a, P: 'a> AsMut<ImageSliceMut<'a, P>> for Image<P> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut ImageSliceMut<'a, P> {
        unsafe {
            &mut *(self as *mut Image<P> as *mut ImageSliceMut<P>)
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

impl<'a, Pixel: 'a> ImageSlice<'a, Pixel> {
    #[inline(always)]
    pub fn clone_map<F, R>(&self, mut f: F) -> Image<R>
    where F: FnMut(slice::Iter<'a, Pixel>) -> Vec<R> {
        Image {
            dimensions: self.dimensions,
            pixels: f(self.pixels.iter()),
        }
    }

    #[inline(always)]
    pub fn pixel_at(&self, x: usize, y: usize) -> &Pixel {
        check_x!(x, self);
        check_y!(y, self);
        &self.pixels[x + y * self.dimensions.pitch]
    }

    #[inline(always)]
    pub fn row(&self, y: usize) -> &[Pixel] {
        check_y!(y, self);
        let i = y * self.dimensions.pitch;
        &self.pixels[i..i + self.dimensions.width]
    }

    #[inline(always)]
    pub fn row_slice(&self, y: usize, left: usize, right: usize) -> &[Pixel] {
        check_y!(y, self);
        check_x!(left, self);
        check_x!(left + right, self);
        let i = y * self.dimensions.pitch;
        &self.pixels[i + left..i + self.dimensions.width - right]
    }

    pub fn min(&self) -> &Pixel
    where Pixel: PartialOrd {
        self.pixels.iter().min_by(|a,b| a.partial_cmp(b).unwrap()).unwrap()
    }

    pub fn max(&self) -> &Pixel
    where Pixel: PartialOrd {
        self.pixels.iter().max_by(|a,b| a.partial_cmp(b).unwrap()).unwrap()
    }

    pub fn crop(&self, x: usize, y: usize, width: usize, height: usize) -> ImageSlice<'a, Pixel> {
        assert!(x + width <= self.dimensions.width, "x too big: {}", x);
        assert!(y + height <= self.dimensions.height);
        ImageSlice {
            dimensions: ImageDimensions {
                width,
                height,
                pitch: self.dimensions.pitch,
            },
            pixels: &self.pixels[x + y * self.dimensions.pitch..],
        }
    }

    pub fn center_crop(&self, width: usize, height: usize) -> ImageSlice<'a, Pixel> {
        self.crop((self.dimensions.width - width) / 2, (self.dimensions.height - height) / 2, width, height)
    }
}

impl<'a, Pixel: 'a> ImageSliceMut<'a, Pixel> {
    #[inline(always)]
    pub fn pixel_at_mut(&mut self, x: usize, y: usize) -> &mut Pixel {
        check_x!(x, self);
        check_y!(y, self);
        &mut self.pixels[x + y * self.dimensions.pitch]
    }
}

//impl<'a, P> Deref for Image<P> {
    //type Target = ImageSlice<'a, P>;

    //fn deref(&'a self) -> &ImageSlice<'a, P> {
        //&*(self as *const Image<P> as *const ImageSlice<P>)
    //}
//}

impl<'a, P> fmt::Debug for ImageSlice<'a, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[image {:?}]", self.dimensions)
    }
}

impl<P> fmt::Debug for Image<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<'a, P> fmt::Debug for ImageSliceMut<'a, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<'a, P: AddAssign + Copy> Add<ImageSlice<'a, P>> for Image<P> {
    type Output = Self;
    fn add(mut self, rhs: ImageSlice<'a, P>) -> Self {
        self.as_mut().add_assign(rhs);
        self
    }
}

impl<'a, P: AddAssign + Copy> AddAssign<ImageSlice<'a, P>> for ImageSliceMut<'a, P> {
    fn add_assign(&mut self, rhs: ImageSlice<'a, P>) {
        assert_eq!(self.dimensions.width, rhs.dimensions.width);
        assert_eq!(self.dimensions.height, rhs.dimensions.height);
        for (l, &r) in self.pixels.iter_mut().zip(rhs.pixels.iter()) {
            l.add_assign(r);
        }
    }
}

impl<P: DivAssign + Copy> Div<P> for Image<P> {
    type Output = Self;
    fn div(mut self, rhs: P) -> Self {
        self.as_mut().div_assign(rhs);
        self
    }
}

impl<'a, P: DivAssign + Copy> DivAssign<P> for ImageSliceMut<'a, P> {
    fn div_assign(&mut self, rhs: P) {
        for p in self.pixels.iter_mut() {
            *p /= rhs;
        }
    }
}

impl<'a, P: DivAssign + Copy> Div<ImageSlice<'a, P>> for Image<P> {
    type Output = Self;
    fn div(mut self, rhs: ImageSlice<'a, P>) -> Self {
        self.as_mut().div_assign(rhs);
        self
    }
}

impl<'a, P: DivAssign + Copy> DivAssign<ImageSlice<'a, P>> for ImageSliceMut<'a, P> {
    fn div_assign(&mut self, rhs: ImageSlice<'a, P>) {
        for (left, right) in self.pixels.iter_mut().zip(rhs.pixels.iter()) {
            *left /= *right;
        }
    }
}

impl<P: Copy + Clone + Default> Image<P> {
    pub fn new(width: usize, height: usize) -> Self {
        let mut pixels = Vec::with_capacity(width * height);
        let zero: P = Default::default();
        pixels.extend(repeat(zero).take(width * height));
        Image {
            dimensions: ImageDimensions {
                width: width,
                height: height,
                pitch: width,
            },
            pixels: pixels,
        }
    }
}

impl<P: Rand> Image<P> {
    pub fn random(width: usize, height: usize) -> Self {
        Image {
            dimensions: ImageDimensions {
                width: width,
                height: height,
                pitch: width,
            },
            pixels: rand::thread_rng().gen_iter().take(width * height).collect()
        }
    }
}

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
        let mut image = Image {
            dimensions,
            pixels: pixels.clone(),
        };
        macro test_ref($r:expr) {
            assert_eq!(dimensions, $r.dimensions);
            assert_eq!(&pixels[..], &$r.pixels[..]);
        }
        test_ref!(image.as_ref());
        test_ref!(image.as_ref().as_ref());
        test_ref!(image.as_mut());
        test_ref!(image.as_mut().as_ref());
        test_ref!(image.as_mut().as_mut());
    }
}
