use magick::*;
use pgm;
use dcraw;
use std::default::Default;
use std::f32;
use std::u16;
use std::fmt;
use std::iter::repeat;
use convert::convert_vec;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::u8;
use std::ops::{AddAssign, DivAssign, Mul};
use turbojpeg;
use rand::{self, Rng, Rand};

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

    //pub fn open_pgm(path: &str) -> Self {
        //let (width, height, data) = pgm::read_from_file(path);
        //let data = if let pgm::Format::F32(d) = data { d } else {
            //panic!("bad format")
        //};
        //Image {
            //width: width,
            //height: height,
            //pixels: data,
        //}
    //}

    pub fn open_jpeg_file<P: AsRef<Path>>(path: P) -> Self {
        Image::<Rgb<f32>>::open_jpeg_file(path).to_gray()
    }

    pub fn save(&self, path: &str) {
        magick_convert(&self.pixels[..], self.width, self.height, "gray", "grayscale", path);
    }

    //pub fn save_pgm(&self, path: &str) {
        //let data = pgm::Format::F32(self.pixels.clone());
        //pgm::write_to_file(path, self.width, self.height, data);
    //}

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

    pub fn to_f32(&self) -> Image<f32> {
        let max = u16::MAX as f32;
        let pixels = self.pixels.iter().map(|&v| {
            v as f32 / max
        }).collect();
        Image {
            width: self.width,
            height: self.height,
            pixels: pixels,
        }
    }

    pub fn to_rggb(&self) -> Image<RgbBayer> {
        let max = u16::MAX as f32;
        let out = self.pixels.iter().enumerate().map(|(i, &gray)| {
            let x = i % self.width;
            let y = (i - x) / self.height;
            let x = x % 2;
            let y = y % 2;
            let mut pix: RgbBayer = Default::default();
            {
                let (v, vc) = match (x, y) {
                    (0, 0) => (&mut pix.r, &mut pix.rc),
                    (1, 0) => (&mut pix.g, &mut pix.gc),
                    (0, 1) => (&mut pix.g, &mut pix.gc),
                    (1, 1) => (&mut pix.b, &mut pix.bc),
                    _ => panic!("invalid bayer coords: {},{}", x, y)
                };
                *v = gray as f32 / max;
                *vc = 1.0;
            }
            pix
        }).collect();
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

impl<P: Rand> Image<P> {
    pub fn random(width: usize, height: usize) -> Self {
        Image {
            width: width,
            height: height,
            pixels: rand::thread_rng().gen_iter().take(width * height).collect()
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Default)]
pub struct Rgb<T> {
    r: T,
    g: T,
    b: T,
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


#[derive(Copy, Clone, PartialEq, Default)]
pub struct RgbBayer {
    r: f32,
    g: f32,
    b: f32,
    rc: f32,
    gc: f32,
    bc: f32,
}

impl AddAssign for RgbBayer {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.rc += rhs.rc;
        self.gc += rhs.gc;
        self.bc += rhs.bc;
    }
}

impl DivAssign<f32> for RgbBayer {
    fn div_assign(&mut self, rhs: f32) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
        self.rc /= rhs;
        self.gc /= rhs;
        self.bc /= rhs;
    }
}

impl Mul<f32> for RgbBayer {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
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

impl Image<RgbBayer> {
    pub fn to_green(&self) -> Image<f32> {
        let pixels = self.pixels.iter().map(|p| {
            if p.gc == 0.0 { 0.0 } else { p.g / p.gc }
        }).collect();
        Image {
            width: self.width,
            height: self.height,
            pixels: pixels,
        }
    }

    pub fn to_rgb(&self) -> Image<Rgb<f32>> {
        let pixels = self.pixels.iter().map(|p| {
            Rgb {
                r: if p.rc == 0.0 { 0.0 } else { p.r / p.rc },
                g: if p.gc == 0.0 { 0.0 } else { p.g / p.gc },
                b: if p.bc == 0.0 { 0.0 } else { p.b / p.bc },
            }
        }).collect();
        Image {
            width: self.width,
            height: self.height,
            pixels: pixels,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_to_gray(b: &mut Bencher) {
        let image = Image::<Rgb<f32>>::random(5000, 4000);
        b.iter(|| {
            image.to_gray()
        });
    }

    #[bench]
    fn bench_crop(b: &mut Bencher) {
        let image = Image::<Rgb<f32>>::random(5000, 4000);
        b.iter(|| {
            image.center_crop(900, 900)
        });
    }
}
