use std::u16;
use image::Image;
use rgb_bayer::RgbBayer;
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
