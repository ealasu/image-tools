use std::u16;
use image::Image;
use dcraw;

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
}
