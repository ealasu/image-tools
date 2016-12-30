use magick::*;
use pgm;
use dcraw;
use std::default::Default;
use std::f32;
use std::fmt;
use std::iter::repeat;
use convert::convert_vec;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::u8;
use turbojpeg;

#[derive(Clone)]
pub struct Image<P> {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<P>,
}

impl<P> fmt::Debug for Image<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[image {}x{}]", self.width, self.height)
    }
}

impl<P> Image<P> {
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

impl Image<f32> {
    pub fn open(path: &str) -> Self {
        let (width, height, data) = magick_stream(path, "gray");
        Image {
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
        Image {
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

    pub fn average(&self) -> f32 {
        self.pixels.iter().fold(0.0, |acc, v| acc + v) / self.pixels.len() as f32
    }

    pub fn min(&self) -> f32 {
        self.pixels.iter().fold(f32::MAX, |acc, &v| acc.min(v))
    }

    pub fn max(&self) -> f32 {
        self.pixels.iter().fold(f32::MIN, |acc, &v| acc.max(v))
    }
}

impl Image<u16> {
    pub fn open_raw(path: &str) -> Self {
        let (width, height, data) = dcraw::open_raw(path);
        Image {
            width: width,
            height: height,
            pixels: data,
        }
    }

    pub fn rescale_to_f32(&self) -> Image<f32> {
        // TODO: rescale to 0..1?
        let mut out: Vec<f32> = Vec::with_capacity(self.pixels.len());
        for v in self.pixels.iter() {
            out.push(*v as f32);
        }
        Image {
            width: self.width,
            height: self.height,
            pixels: out,
        }
    }
}

impl<P: Copy + Clone + Default> Image<P> {
    pub fn new(width: usize, height: usize) -> Self {
        let mut pixels = Vec::with_capacity(width * height);
        let zero: P = Default::default();
        pixels.extend(repeat(zero).take(width * height));
        Image {
            width: width,
            height: height,
            pixels: pixels,
        }
    }
}

impl<P: Clone> Image<P> {
    pub fn crop(&self, x: usize, y: usize, width: usize, height: usize) -> Image<P> {
        assert!(x + width <= self.width);
        assert!(y + height <= self.height);
        let mut pixels = Vec::with_capacity(width * height);
        for y in y..y + width {
            let start = y * self.width + x;
            let end = start + width;
            pixels.extend_from_slice(&self.pixels[start..end]);
        }
        Image {
            width: width,
            height: height,
            pixels: pixels,
        }
    }

    pub fn center_crop(&self, width: usize, height: usize) -> Image<P> {
        self.crop((self.width - width) / 2, (self.height - height) / 2, width, height)
    }
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct Rgb<T> {
    r: T,
    g: T,
    b: T,
}

impl Image<Rgb<f32>> {
    pub fn open(path: &str) -> Self {
        let (width, height, data) = magick_stream(path, "rgb");
        let pixels = convert_vec(data);
        assert_eq!(pixels.len(), width * height);
        Image {
            width: width,
            height: height,
            pixels: pixels,
        }
    }

    pub fn open_jpeg_data(data: &[u8]) -> Self {
        let image = turbojpeg::decompress(data);
        let max = u8::MAX as f32;
        Image {
            width: image.width as usize,
            height: image.height as usize,
            pixels: image.pixels.iter().map(|p| {
                Rgb {
                    r: p.r as f32 / max,
                    g: p.g as f32 / max,
                    b: p.b as f32 / max,
                }
            }).collect(),
        }
    }

    pub fn open_jpeg_file<P: AsRef<Path>>(path: P) -> Self {
        let mut f = File::open(path).unwrap();
        let mut data = vec![];
        f.read_to_end(&mut data).unwrap();
        Self::open_jpeg_data(&data)
    }

    pub fn save(&self, path: &str) {
        let data = convert_vec(self.pixels.clone());
        magick_convert(&data, self.width, self.height, "rgb", "truecolor", path);
    }

    pub fn to_gray(&self) -> Image<f32> {
        let pixels = self.pixels.iter().map(|p| {
            (p.r + p.g + p.b) / 3.0
        }).collect();
        Image {
            width: self.width,
            height: self.height,
            pixels: pixels,
        }
    }
}
