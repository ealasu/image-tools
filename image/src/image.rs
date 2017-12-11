use std::default::Default;
use std::fmt;
use std::iter::repeat;
use std::ops::*;
use std::slice;
use rand::{self, Rng, Rand};

#[derive(Copy, Clone, Debug)]
pub struct ImageDimensions {
    pub width: usize,
    pub height: usize,
    pub pitch: usize,
}

macro_rules! check_x_y {
    ($x:expr, $y:expr, $dim:expr) => {
        {
            debug_assert!($x < $dim.width, "x ({}) >= width ({})", $x, $dim.width);
            debug_assert!($y < $dim.height, "y ({}) >= height ({})", $y, $dim.height);
        }
    }
}

pub trait ImageSlice<'a> {
    type Pixel;
    fn pixels(&'a self) -> &[Self::Pixel];
    fn dimensions(&'a self) -> ImageDimensions;

    #[inline(always)]
    fn clone_map<F, P>(&'a self, mut f: F) -> Image<P>
    where F: FnMut(slice::Iter<'a, Self::Pixel>) -> Vec<P> {
        Image {
            dimensions: self.dimensions(),
            pixels: f(self.pixels().iter()),
        }
    }

    #[inline(always)]
    fn pixel_at(&'a self, x: usize, y: usize) -> &'a Self::Pixel {
        check_x_y!(x, y, self.dimensions());
        &self.pixels()[x + y * self.dimensions().pitch]
    }

    #[inline(always)]
    fn row(&'a self, y: usize) -> &[Self::Pixel] {
        let i = y * self.dimensions().pitch;
        &self.pixels()[i..i + self.dimensions().width]
    }

    #[inline(always)]
    fn row_slice(&'a self, y: usize, left: usize, right: usize) -> &[Self::Pixel] {
        let w = self.dimensions().width;
        let i = y * w;
        &self.pixels()[i + left..i + w - right]
    }
}

pub trait ImageSliceMut<'a>: ImageSlice<'a> {
    fn pixels_mut(&'a mut self) -> &mut [Self::Pixel];

    #[inline(always)]
    fn pixel_at_mut(&mut self, x: usize, y: usize) -> &mut Self::Pixel {
        //check_x_y!(x, y, self.dimensions());
        let p = {
            let v = self.dimensions().pitch;
            v
        };
        &mut self.pixels_mut()[x + y * p]
    }
}

pub struct Image<P> {
    dimensions: ImageDimensions,
    pixels: Vec<P>,
}

impl <'a, P> ImageSlice<'a> for Image<P> {
    type Pixel = P;

    #[inline(always)]
    fn dimensions(&self) -> ImageDimensions { self.dimensions }

    #[inline(always)]
    fn pixels(&'a self) -> &[Self::Pixel] {
        &self.pixels
    }
}

impl <'a, P> ImageSliceMut<'a> for Image<P> {
    #[inline(always)]
    fn pixels_mut(&'a mut self) -> &mut [Self::Pixel] {
        &mut self.pixels
    }
}



//pub trait Image {
    //type Pixel;

    //fn width(&self) -> usize;
    //fn height(&self) -> usize;
    //fn pitch(&self) -> usize;
    //fn pixels(&self) -> &[Self::Pixel];


    //pub fn map<F,R>(&self, f: F) -> Image<R> where F: FnMut(&P) -> R {
        //let pixels = self.pixels.iter().map(f).collect();
        //Image {
            //width: self.width,
            //height: self.height,
            //pixels: pixels,
        //}
    //}
//}

//impl fmt::Debug for Image {
    //fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //write!(f, "[image {}x{}]", self.width(), self.height())
    //}
//}

//}

//impl<P: AddAssign> Add for Image<P> {
    //type Output = Self;
    //fn add(mut self, rhs: Self) -> Self {
        //self += rhs;
        //self
    //}
//}

//impl<P: AddAssign> AddAssign for Image<P> {
    //fn add_assign(&mut self, rhs: Self) {
        //assert_eq!(self.width, rhs.width);
        //assert_eq!(self.height, rhs.height);
        //for (l,r) in self.pixels.iter_mut().zip(rhs.pixels.into_iter()) {
            //l.add_assign(r);
        //}
    //}
//}

//impl<P: DivAssign + Copy> Div<P> for Image<P> {
    //type Output = Self;
    //fn div(mut self, rhs: P) -> Self {
        //self /= rhs;
        //self
    //}
//}

//impl<P: DivAssign + Copy> DivAssign<P> for Image<P> {
    //fn div_assign(&mut self, rhs: P) {
        //for p in self.pixels.iter_mut() {
            //*p /= rhs;
        //}
    //}
//}

//impl<'a, P: DivAssign + Copy> Div<&'a Image<P>> for Image<P> {
    //type Output = Self;
    //fn div(mut self, rhs: &Self) -> Self {
        //self /= rhs;
        //self
    //}
//}

//impl<'a, P: DivAssign + Copy> DivAssign<&'a Image<P>> for Image<P> {
    //fn div_assign(&mut self, rhs: &Self) {
        //for (left, right) in self.pixels.iter_mut().zip(rhs.pixels.iter()) {
            //*left /= *right;
        //}
    //}
//}

//impl<P: Copy + Clone + Default> Image<P> {
    //pub fn new(width: usize, height: usize) -> Self {
        //let mut pixels = Vec::with_capacity(width * height);
        //let zero: P = Default::default();
        //pixels.extend(repeat(zero).take(width * height));
        //Image {
            //width: width,
            //height: height,
            //pixels: pixels,
        //}
    //}
//}

//impl<P: Clone> Image<P> {
    //pub fn crop(&self, x: usize, y: usize, width: usize, height: usize) -> Image<P> {
        //assert!(x + width <= self.width, "x too big: {}", x);
        //assert!(y + height <= self.height);
        //let mut pixels = Vec::with_capacity(width * height);
        //for y in y..y + width {
            //let start = y * self.width + x;
            //let end = start + width;
            //pixels.extend_from_slice(&self.pixels[start..end]);
        //}
        //Image {
            //width: width,
            //height: height,
            //pixels: pixels,
        //}
    //}

    //pub fn center_crop(&self, width: usize, height: usize) -> Image<P> {
        //self.crop((self.width - width) / 2, (self.height - height) / 2, width, height)
    //}
//}

//impl<P: Rand> Image<P> {
    //pub fn random(width: usize, height: usize) -> Self {
        //Image {
            //width: width,
            //height: height,
            //pixels: rand::thread_rng().gen_iter().take(width * height).collect()
        //}
    //}
//}

//impl<P: PartialOrd + Copy> Image<P> {
    //pub fn min(&self) -> P {
        //*self.pixels.iter().min_by(|a,b| a.partial_cmp(b).unwrap()).unwrap()
    //}

    //pub fn max(&self) -> P {
        //*self.pixels.iter().max_by(|a,b| a.partial_cmp(b).unwrap()).unwrap()
    //}
//}

