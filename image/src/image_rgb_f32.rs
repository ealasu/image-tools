use std::u8;
use std::f32;
use image::Image;
use rgb::Rgb;
use util::{min, max};
use convert::convert_vec;
use magick::*;

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

    pub fn min_max(&self) -> (Rgb<f32>, Rgb<f32>) {
        let mut min = Rgb {
            r: f32::MAX,
            g: f32::MAX,
            b: f32::MAX,
        };
        let mut max = Rgb {
            r: f32::MIN,
            g: f32::MIN,
            b: f32::MIN,
        };
        for p in self.pixels.iter() {
            if p.r < min.r { min.r = p.r; }
            if p.g < min.g { min.g = p.g; }
            if p.b < min.b { min.b = p.b; }
            if p.r > max.r { max.r = p.r; }
            if p.g > max.g { max.g = p.g; }
            if p.b > max.b { max.b = p.b; }
        }
        (min, max)
    }

    pub fn to_u8(&self) -> Image<Rgb<u8>> {
        let (min_p, max_p) = self.min_max();
        let src_min = min(min_p.r, min(min_p.g, min_p.b));
        let src_max = max(max_p.r, max(max_p.g, max_p.b));
        let dst_min = u8::MIN as f32;
        let dst_max = u8::MAX as f32;
        let dst_d = dst_max - dst_min;
        let src_d = src_max - src_min;
        self.map(|p| {
            Rgb {
                r: (((p.r - src_min) * dst_d) / src_d) as u8,
                g: (((p.g - src_min) * dst_d) / src_d) as u8,
                b: (((p.b - src_min) * dst_d) / src_d) as u8,
            }
        })
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
