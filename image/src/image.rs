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
use std::io::{BufReader, BufWriter};
use std::u8;
use std::ops::{AddAssign, DivAssign, Add, Mul, Div};
use turbojpeg;
use rand::{self, Rng, Rand};
use fits;

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

    pub fn map<F,R>(&self, f: F) -> Image<R> where F: FnMut(&P) -> R {
        let pixels = self.pixels.iter().map(f).collect();
        Image {
            width: self.width,
            height: self.height,
            pixels: pixels,
        }
    }
}

impl<P: AddAssign> Add for Image<P> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl<P: AddAssign> AddAssign for Image<P> {
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(self.width, rhs.width);
        assert_eq!(self.height, rhs.height);
        for (l,r) in self.pixels.iter_mut().zip(rhs.pixels.into_iter()) {
            l.add_assign(r);
        }
    }
}

impl<P: DivAssign + Copy> Div<P> for Image<P> {
    type Output = Self;
    fn div(mut self, rhs: P) -> Self {
        self /= rhs;
        self
    }
}

impl<P: DivAssign + Copy> DivAssign<P> for Image<P> {
    fn div_assign(&mut self, rhs: P) {
        for p in self.pixels.iter_mut() {
            *p /= rhs;
        }
    }
}

impl<'a, P: DivAssign + Copy> Div<&'a Image<P>> for Image<P> {
    type Output = Self;
    fn div(mut self, rhs: &Self) -> Self {
        self /= rhs;
        self
    }
}

impl<'a, P: DivAssign + Copy> DivAssign<&'a Image<P>> for Image<P> {
    fn div_assign(&mut self, rhs: &Self) {
        for (left, right) in self.pixels.iter_mut().zip(rhs.pixels.iter()) {
            *left /= *right;
        }
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

    pub fn open_fits(path: &str) -> Self {
        let mut r = BufReader::new(File::open(path).unwrap());
        let (w, h, data) = fits::read_image(&mut r);
        let pixels = match data {
            fits::Data::F32(v) => v,
            _ => panic!()
        };
        Image {
            width: w,
            height: h,
            pixels: pixels,
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
        Image::<Rgb<u8>>::open_jpeg_file(path).to_f32().to_gray()
    }

    pub fn save(&self, path: &str) {
        magick_convert(&self.pixels[..], self.width, self.height, "gray", "grayscale", path);
    }

    pub fn save_fits(&self, filename: &str) {
        let mut f = BufWriter::new(File::create(filename).unwrap());
        fits::write_image(&mut f, self.width, self.height, &fits::Data::F32(self.pixels.clone()));
    }

    //pub fn save_pgm(&self, path: &str) {
        //let data = pgm::Format::F32(self.pixels.clone());
        //pgm::write_to_file(path, self.width, self.height, data);
    //}

    pub fn average(&self) -> f32 {
        self.pixels.iter().fold(0.0, |acc, v| acc + v) / self.pixels.len() as f32
    }

    pub fn to_u16(&self) -> Image<u16> {
        let src_min = self.pixels.iter().fold(f32::MAX, |acc, &v| acc.min(v));
        let src_max = self.pixels.iter().fold(f32::MIN, |acc, &v| acc.max(v));
        let src_d = src_max - src_min;
        let dst_min = u16::MIN as f32;
        let dst_max = u16::MAX as f32;
        let dst_d = dst_max - dst_min;
        self.map(|&p| (((p - src_min) * dst_d) / src_d) as u16)
    }

    pub fn to_u8(&self) -> Image<u8> {
        let src_min = self.pixels.iter().fold(f32::MAX, |acc, &v| acc.min(v));
        let src_max = self.pixels.iter().fold(f32::MIN, |acc, &v| acc.max(v));
        let src_d = src_max - src_min;
        let dst_min = u8::MIN as f32;
        let dst_max = u8::MAX as f32;
        let dst_d = dst_max - dst_min;
        self.map(|&p| (((p - src_min) * dst_d) / src_d) as u8)
    }

    pub fn to_rggb(&self) -> Image<RgbBayer> {
        let mut pixels = Vec::with_capacity(self.width * self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let gray = *self.pixel_at(x, y);
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
                    *v = gray;
                    *vc = 1.0;
                }
                pixels.push(pix);
            }
        }
        Image {
            width: self.width,
            height: self.height,
            pixels: pixels,
        }
    }
}

impl Image<u8> {
    pub fn to_rgb(&self) -> Image<Rgb<u8>> {
        self.map(|&p| Rgb { r: p, g: p, b: p })
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
        let mut pixels = Vec::with_capacity(self.width * self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let gray = *self.pixel_at(x, y);
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
                pixels.push(pix);
            }
        }
        Image {
            width: self.width,
            height: self.height,
            pixels: pixels,
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

impl Image<Rgb<u8>> {
    pub fn to_f32(&self) -> Image<Rgb<f32>> {
        let max = u8::MAX as f32;
        Image {
            width: self.width,
            height: self.height,
            pixels: self.pixels.iter().map(|p| {
                Rgb {
                    r: p.r as f32 / max,
                    g: p.g as f32 / max,
                    b: p.b as f32 / max,
                }
            }).collect()
        }
    }

    pub fn open_jpeg_data(data: &[u8]) -> Self {
        let image = turbojpeg::decompress(data);
        Image {
            width: image.width,
            height: image.height,
            pixels: image.pixels.iter().map(|p| {
                Rgb {
                    r: p.r,
                    g: p.g,
                    b: p.b,
                }
            }).collect()
        }
    }

    pub fn open_jpeg_file<P: AsRef<Path>>(path: P) -> Self {
        let mut f = File::open(path).unwrap();
        let mut data = vec![];
        f.read_to_end(&mut data).unwrap();
        Self::open_jpeg_data(&data)
    }

    pub fn save_jpeg(&self) -> Vec<u8> {
        let image = turbojpeg::Image {
            width: self.width,
            height: self.height,
            pixels: self.pixels.iter().map(|p| {
                turbojpeg::Pixel {
                    r: p.r,
                    g: p.g,
                    b: p.b,
                }
            }).collect()
        };
        turbojpeg::compress(&image)
    }

    pub fn save_jpeg_file<P: AsRef<Path>>(&self, path: P) {
        let data = self.save_jpeg();
        let mut f = File::create(path).unwrap();
        f.write_all(&data).unwrap();
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

    pub fn min_max(&self) -> ((f32, f32, f32), (f32, f32, f32)) {
        let mut min_r = f32::MAX;
        let mut min_g = f32::MAX;
        let mut min_b = f32::MAX;
        let mut max_r = f32::MIN;
        let mut max_g = f32::MIN;
        let mut max_b = f32::MIN;
        for p in self.pixels.iter() {
            if p.r < min_r { min_r = p.r; }
            if p.g < min_g { min_g = p.g; }
            if p.b < min_b { min_b = p.b; }
            if p.r > max_r { max_r = p.r; }
            if p.g > max_g { max_g = p.g; }
            if p.b > max_b { max_b = p.b; }
        }
        ((min_r, min_g, min_b), (max_r, max_g, max_b))
    }

    pub fn to_u8(&self) -> Image<Rgb<u8>> {
        let ((min_r, min_g, min_b), (max_r, max_g, max_b)) = self.min_max();
        let dst_min = u8::MIN as f32;
        let dst_max = u8::MAX as f32;
        let dst_d = dst_max - dst_min;
        self.map(|p| {
            Rgb {
                r: (((p.r - min_r) * dst_d) / (max_r - min_r)) as u8,
                g: (((p.g - min_g) * dst_d) / (max_g - min_g)) as u8,
                b: (((p.b - min_b) * dst_d) / (max_b - min_b)) as u8,
            }
        })
    }
}

impl<T: PartialOrd + Copy> Image<T> {
    pub fn min(&self) -> T {
        *self.pixels.iter().min_by(|a,b| a.partial_cmp(b).unwrap()).unwrap()
    }

    pub fn max(&self) -> T {
        *self.pixels.iter().max_by(|a,b| a.partial_cmp(b).unwrap()).unwrap()
    }
}

#[derive(Copy, Clone, PartialEq, Default)]
pub struct RgbBayer {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub rc: f32,
    pub gc: f32,
    pub bc: f32,
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
        self.map(|p| {
            if p.gc == 0.0 { 0.0 } else { p.g / p.gc }
        })
    }

    pub fn to_green_interpolated(&self) -> Image<f32> {
        let mut pixels = Vec::with_capacity(self.width * self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let p = *self.pixel_at(x, y);
                let gray = if p.gc > 0.0 {
                    p.g
                } else {
                    let left = if x == 0 { 0.0 } else { self.pixel_at(x - 1, y).g };
                    let right = if x == self.width - 1 { 0.0 } else { self.pixel_at(x + 1, y).g };
                    let top = if y == 0 { 0.0 } else { self.pixel_at(x, y - 1).g };
                    let bottom = if y == self.height - 1 { 0.0 } else { self.pixel_at(x, y + 1).g };
                    (left + right + top + bottom) / 4.0
                };
                pixels.push(gray);
            }
        }
        Image {
            width: self.width,
            height: self.height,
            pixels: pixels,
        }
    }

    pub fn to_rgb(&self) -> Image<Rgb<f32>> {
        self.map(|p| {
            Rgb {
                r: if p.rc == 0.0 { 0.0 } else { p.r / p.rc },
                g: if p.gc == 0.0 { 0.0 } else { p.g / p.gc },
                b: if p.bc == 0.0 { 0.0 } else { p.b / p.bc },
            }
        })
    }

    pub fn holes(&self) -> Image<Rgb<f32>> {
        self.map(|p| {
            Rgb {
                r: p.rc,
                g: p.gc / 2.0,
                b: p.bc,
            }
        })
    }

    pub fn correct_white_balance(&self) -> Image<RgbBayer> {
        let (avg_r, avg_g, avg_b) = self.avg();
        let m_r = avg_g / avg_r;
        let m_b = avg_g / avg_b;
        self.map(|p| {
            RgbBayer {
                r: p.r * m_r,
                rc: p.rc,
                g: p.g,
                gc: p.gc,
                b: p.b * m_b,
                bc: p.bc,
            }
        })
    }

    /// Computes the average of each component separately.
    /// Returns a tuple of `(red_avg, green_avg, blue_avg)`.
    pub fn avg(&self) -> (f32, f32, f32) {
        let mut sum_r = 0.0;
        let mut count_r = 0.0;
        let mut sum_g = 0.0;
        let mut count_g = 0.0;
        let mut sum_b = 0.0;
        let mut count_b = 0.0;
        for p in self.pixels.iter() {
            sum_r += p.r * p.rc;
            count_r += p.rc;
            sum_g += p.g * p.gc;
            count_g += p.gc;
            sum_b += p.b * p.bc;
            count_b += p.bc;
        }
        let avg_r = sum_r / count_r;
        let avg_g = sum_g / count_g;
        let avg_b = sum_b / count_b;
        (avg_r, avg_g, avg_b)
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
