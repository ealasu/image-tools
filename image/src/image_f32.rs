use std::fs::File;
use std::path::Path;
use std::u8;
use std::f32;
use std::u16;
use std::io::{BufReader, BufWriter};
use image::Image;
use fits;
use magick::*;
use rgb::Rgb;
use rgb_bayer::RgbBayer;

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
        let (shape, data) = fits::read_image(&mut r);
        assert_eq!(shape.len(), 2);
        let w = shape[0];
        let h = shape[1];
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

    pub fn open_jpeg_file<P: AsRef<Path>>(path: P) -> Self {
        Image::<Rgb<u8>>::open_jpeg_file(path).to_f32().to_gray()
    }

    pub fn save(&self, path: &str) {
        magick_convert(&self.pixels[..], self.width, self.height, "gray", "grayscale", path);
    }

    pub fn save_fits(&self, filename: &str) {
        let mut f = BufWriter::new(File::create(filename).unwrap());
        let shape = [self.width, self.height];
        fits::write_image(&mut f, &shape[..], &fits::Data::F32(self.pixels.clone()));
    }

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
