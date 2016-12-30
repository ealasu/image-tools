use magick::*;
use pgm;
use dcraw;
use std::default::Default;
use std::f32;
use std::fmt;
use std::iter::repeat;

#[derive(Clone)]
pub struct GrayImage<P> {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<P>,
}

impl<P> fmt::Debug for GrayImage<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[image {}x{}]", self.width, self.height)
    }
}

impl<P> GrayImage<P> {
    #[inline(always)]
    pub fn pixel_at(&self, x: usize, y: usize) -> &P {
        //assert!(x < self.width);
        //assert!(y < self.height);
        &self.pixels[x + y * self.width]
    }

    #[inline(always)]
    pub fn pixel_at_mut(&mut self, x: usize, y: usize) -> &mut P {
        //assert!(x < self.width);
        //assert!(y < self.height);
        &mut self.pixels[x + y * self.width]
    }
}

impl GrayImage<f32> {
    pub fn open(path: &str) -> Self {
        let (width, height, data) = magick_stream(path, "gray");
        GrayImage {
            width: width,
            height: height,
            pixels: data,
        }
    }

    pub fn open_pgm(path: &str) -> Self {
        let (width, height, data) = pgm::read_from_file(path);
        let data = if let pgm::Format::F32(d) = data { d } else {
            panic!("bad format")
        };
        GrayImage {
            width: width,
            height: height,
            pixels: data,
        }
    }

    pub fn save(&self, path: &str) {
        magick_convert(&self.pixels[..], self.width, self.height, "gray", "grayscale", path);
    }

    pub fn save_pgm(&self, path: &str) {
        let data = pgm::Format::F32(self.pixels.clone());
        pgm::write_to_file(path, self.width, self.height, data);
    }
}

impl GrayImage<u16> {
    pub fn open_raw(path: &str) -> Self {
        let (width, height, data) = dcraw::open_raw(path);
        GrayImage {
            width: width,
            height: height,
            pixels: data,
        }
    }

    pub fn rescale_to_f32(&self) -> GrayImage<f32> {
        // TODO: rescale to 0..1?
        let mut out: Vec<f32> = Vec::with_capacity(self.pixels.len());
        for v in self.pixels.iter() {
            out.push(*v as f32);
        }
        GrayImage {
            width: self.width,
            height: self.height,
            pixels: out,
        }
    }
}

impl<P: Copy + Clone + Default> GrayImage<P> {
    fn new(width: usize, height: usize) -> Self {
        let mut pixels = Vec::with_capacity(width * height);
        let zero: P = Default::default();
        pixels.extend(repeat(zero).take(width * height));
        GrayImage {
            width: width,
            height: height,
            pixels: pixels,
        }
    }
}
