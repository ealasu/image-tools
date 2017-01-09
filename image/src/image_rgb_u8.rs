use std::fs::File;
use std::u8;
use std::io::prelude::*;
use std::path::Path;
use image::Image;
use rgb::Rgb;
use turbojpeg;

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
