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
        self.map(|&p| {
            p as f32 / max
        })
    }

    pub fn into_f64(&self) -> Image<f64> {
        let max = u16::MAX as f64;
        self.map(|&p| {
            p as f64 / max
        })
    }
}
