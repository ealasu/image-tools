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
use num::Float;

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
}

impl Image<Rgb<f64>> {
    pub fn save_fits(&self, filename: &str) {
        let data = convert_vec(self.pixels.clone());
        let mut f = BufWriter::new(File::create(filename).unwrap());
        let shape = [3, self.width, self.height];
        fits::write_image(&mut f, &shape[..], &fits::Data::F64(data));
    }
}

impl<P: Float> Image<Rgb<P>> {
    pub fn to_gray(&self) -> Image<P> {
        let three = P::one() + P::one() + P::one();
        self.map(|p| {
            (p.r + p.g + p.b) / three
        })
    }

    pub fn min_max(&self) -> (Rgb<P>, Rgb<P>) {
        let mut min = Rgb {
            r: P::max_value(),
            g: P::max_value(),
            b: P::max_value(),
        };
        let mut max = Rgb {
            r: P::min_value(),
            g: P::min_value(),
            b: P::min_value(),
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

    pub fn stretch(&self, dst_min: P, dst_max: P) -> Image<Rgb<P>> {
        let (min_p, max_p) = self.min_max();
        let src_min = min_p.r.min(min_p.g.min(min_p.b));
        let src_max = max_p.r.max(max_p.g.max(max_p.b));
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
        self.stretch(P::from(u8::MIN).unwrap(), P::from(u8::MAX).unwrap()).map(|p| {
            Rgb {
                r: p.r.to_u8().unwrap(),
                g: p.g.to_u8().unwrap(),
                b: p.b.to_u8().unwrap(),
            }
        })
    }

    pub fn median(&self) -> Rgb<P> {
        fn median_by<P,F>(pixels: &[Rgb<P>], f: F) -> P
        where P: Float, F: Fn(&Rgb<P>) -> P {
            let mut pixels = pixels.iter().map(f).collect::<Vec<_>>();
            let two = P::one() + P::one();
            sort_floats(&mut pixels[..]);
            pixels[pixels.len() / two.to_usize().unwrap()]
        }
        Rgb {
            r: median_by(&self.pixels, |p| p.r),
            g: median_by(&self.pixels, |p| p.g),
            b: median_by(&self.pixels, |p| p.b),
        }
    }

    pub fn remove_background(&self, amount: P) -> Image<Rgb<P>> {
        let median = self.median() * amount;
        self.map(|&p| {
            Rgb {
                r: P::zero().max(p.r - median.r),
                g: P::zero().max(p.g - median.g),
                b: P::zero().max(p.b - median.b),
            }
        })
    }

    pub fn gamma(&self, amount: P) -> Image<Rgb<P>> {
        let f = |v: P| {
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
