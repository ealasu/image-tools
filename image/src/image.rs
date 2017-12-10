use std::default::Default;
use std::fmt;
use std::iter::repeat;
use std::ops::*;
use std::slice;
use rand::{self, Rng, Rand};

// owned image : Vec<Pixel>
// slice : &[Pixel]
// view (pixel iter) : Iterator<Pixel>

#[derive(Copy, Clone, Debug)]
pub struct ImageDimensions {
    pub width: usize,
    pub height: usize,
    pub pitch: usize,
}

pub trait DimensionedImage {
    fn dimensions(&self) -> ImageDimensions;
}

impl<'a, T> DimensionedImage for &'a T where T: DimensionedImage + ?Sized {
    #[inline(always)]
    fn dimensions(&self) -> ImageDimensions { (**self).dimensions() }
}

impl<'a, T> DimensionedImage for &'a mut T where T: DimensionedImage + ?Sized {
    #[inline(always)]
    fn dimensions(&self) -> ImageDimensions { (**self).dimensions() }
}

pub trait ImageIter<'a>: DimensionedImage {
    type Pixel: 'a;
    type Iterator: Iterator<Item=Self::Pixel> + 'a;
    fn pixels_iter(self) -> Self::Iterator;
}

//pub trait ImageIterMut<'a>: DimensionedImage {
    //type Pixel: 'a;
    //type Iterator: Iterator<Item=&'a mut Self::Pixel> + 'a;
    //fn pixels_iter_mut(&'a mut self) -> Self::Iterator;
//}

//impl<'a, P> ImageIter<'a> for ImageIterMut<'a, Pixel=P> {
    //type Pixel = P;
    ////type Iterator = // TODO
    //fn pixels_iter(&'a self) -> Self::Iterator {
        //self.pixels_iter().map(|p| *p)
    //}
//}

pub trait ImageSlice<'a>: DimensionedImage {
    type Pixel;
    fn pixels(&'a self) -> &[Self::Pixel];
}

pub trait ImageSliceMut<'a>: DimensionedImage + ImageSlice<'a> {
    fn pixels_mut(&'a mut self) -> &mut [Self::Pixel];
}

impl<'a, P: 'a> ImageIter<'a> for &'a ImageSlice<'a, Pixel=P> {
    type Pixel = &'a P;
    type Iterator = slice::Iter<'a, P>;

    #[inline(always)]
    fn pixels_iter(self) -> Self::Iterator {
        self.pixels().iter()
    }
}

//impl<'a, P: 'a> ImageIterMut<'a> for ImageSliceMut<'a, Pixel=P> {
    //type Pixel = P;
    //type Iterator = slice::IterMut<'a, P>;
    //fn pixels_iter_mut(&'a mut self) -> Self::Iterator {
        //self.pixels_mut().iter_mut()
    //}
//}
impl<'a, P: 'a> ImageIter<'a> for &'a mut ImageSliceMut<'a, Pixel=P> {
    type Pixel = &'a mut P;
    type Iterator = slice::IterMut<'a, P>;

    #[inline(always)]
    fn pixels_iter(self) -> Self::Iterator {
        self.pixels_mut().iter_mut()
    }
}


pub struct Image<P> {
    dimensions: ImageDimensions,
    pixels: Vec<P>,
}

impl <P> DimensionedImage for Image<P> {
    #[inline(always)]
    fn dimensions(&self) -> ImageDimensions { self.dimensions }
}

impl <'a, P> ImageSlice<'a> for Image<P> {
    type Pixel = P;

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

    //#[inline(always)]
    //fn pixel_at(&self, x: usize, y: usize) -> &Self::Pixel {
        ////assert!(x < self.width);
        ////assert!(y < self.height);
        //&self.pixels()[x + y * self.pitch()]
    //}

    //#[inline(always)]
    //fn pixel_at_mut(&mut self, x: usize, y: usize) -> &mut Self::Pixel {
        ////assert!(x < self.width);
        ////assert!(y < self.height);
        //&mut self.pixels()[x + y * self.pitch()]
    //}

    //#[inline(always)]
    //fn row(&self, y: usize) -> &[Self::Pixel] {
        //let i = y * self.pitch();
        //&self.pixels()[i..i + self.width()]
    //}

    //#[inline(always)]
    //pub fn row_slice(&self, y: usize, left: usize, right: usize) -> &[P] {
        //let i = y * self.width;
        //&self.pixels[i + left..i + self.width - right]
    //}

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

