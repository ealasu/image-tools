use std::u8;
use std::f32;
use std::io::{BufReader, BufWriter};
use std::fs::File;
use image::Image;
use rgb::Rgb;
use util::{min, max};
use convert::convert_vec;
use magick::*;
use fits;
use quickersort::sort_floats;

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

    pub fn open_fits(path: &str) -> Self {
        let mut r = BufReader::new(File::open(path).unwrap());
        let (shape, data) = fits::read_image(&mut r);
        assert_eq!(shape.len(), 3);
        assert_eq!(shape[0], 3);
        let w = shape[1];
        let h = shape[2];
        let pixels = match data {
            fits::Data::F32(v) => v,
            _ => panic!()
        };
        let pixels = convert_vec(pixels);
        Image {
            width: w,
            height: h,
            pixels: pixels,
        }
    }

    pub fn save_fits(&self, filename: &str) {
        let data = convert_vec(self.pixels.clone());
        let mut f = BufWriter::new(File::create(filename).unwrap());
        let shape = [3, self.width, self.height];
        fits::write_image(&mut f, &shape[..], &fits::Data::F32(data));
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

    pub fn stretch(&self, dst_min: f32, dst_max: f32) -> Image<Rgb<f32>> {
        let (min_p, max_p) = self.min_max();
        let src_min = min(min_p.r, min(min_p.g, min_p.b));
        let src_max = max(max_p.r, max(max_p.g, max_p.b));
        let dst_d = dst_max - dst_min;
        let src_d = src_max - src_min;
        self.map(|p| {
            Rgb {
                r: ((p.r - src_min) * dst_d) / src_d,
                g: ((p.g - src_min) * dst_d) / src_d,
                b: ((p.b - src_min) * dst_d) / src_d,
            }
        })
    }

    pub fn to_u8(&self) -> Image<Rgb<u8>> {
        self.stretch(u8::MIN as f32, u8::MAX as f32).map(|p| {
            Rgb {
                r: p.r as u8,
                g: p.g as u8,
                b: p.b as u8,
            }
        })
    }

    pub fn median(&self) -> Rgb<f32> {
        fn median_by<F>(pixels: &[Rgb<f32>], f: F) -> f32
        where F: Fn(Rgb<f32>) -> f32 {
            let mut pixels = pixels.iter().map(|p| p.r).collect::<Vec<_>>();
            sort_floats(&mut pixels[..]);
            pixels[pixels.len() / 2]
        }
        Rgb {
            r: median_by(&self.pixels, |p| p.r),
            g: median_by(&self.pixels, |p| p.g),
            b: median_by(&self.pixels, |p| p.b),
        }
    }

    pub fn remove_background(&self, amount: f32) -> Image<Rgb<f32>> {
        let median = self.median() * amount;
        self.map(|&p| {
            Rgb {
                r: max(0.0, p.r - median.r),
                g: max(0.0, p.g - median.g),
                b: max(0.0, p.b - median.b),
            }
        })
    }

    pub fn gamma(&self, amount: f32) -> Image<Rgb<f32>> {
        let f = |v: f32| {
            v.powf(amount)
        };
        self.map(|p| {
            Rgb {
                r: f(p.r),
                g: f(p.g),
                b: f(p.b),
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
