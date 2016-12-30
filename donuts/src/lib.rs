#![feature(test)]
#![feature(iter_max_by)]

#[cfg(test)] extern crate test;
#[cfg(test)] extern crate rand;
extern crate statistical;
extern crate quickersort;
extern crate image;

mod remove_background;
mod projection;
mod correlation;

use image::Image;

pub struct Projection {
    x: Vec<f32>,
    y: Vec<f32>,
}

pub fn preprocess_image(mut image: Image<f32>) -> Projection {
    remove_background::remove_background(&mut image, 32);
    Projection {
        x: projection::x_projection(&image),
        y: projection::y_projection(&image),
    }
}

pub fn align(reference: &Projection, sample: &Projection) -> (f32, f32) {
    let n = 100;
    let x = correlation::calc_offset(&reference.x[..], &sample.x[..], n);
    let y = correlation::calc_offset(&reference.y[..], &sample.y[..], n);
    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use rand::{self, Rng};
    use image::Image;

    #[test]
    fn test_end_to_end() {
        let ref_image = Image::<f32>::open("test/ref.jpg");
        let sample_image = Image::<f32>::open("test/sample.jpg");
        let ref_p = preprocess_image(ref_image);
        let sample_p = preprocess_image(sample_image);
        let offset = align(&ref_p, &sample_p);
        assert_eq!(offset, (-15.863456, -12.371683));
    }

    #[bench]
    fn bench_preprocess(b: &mut Bencher) {
        let w = 900;
        let h = 900;
        let image = Image {
            width: w,
            height: h,
            pixels: rand::thread_rng().gen_iter().take(w * h).collect()
        };
        b.iter(|| {
            let image = image.clone();
            preprocess_image(image)
        });
    }

    #[bench]
    fn bench_align(b: &mut Bencher) {
        let w = 900;
        let h = 900;
        let ref_image = Image {
            width: w,
            height: h,
            pixels: rand::thread_rng().gen_iter().take(w * h).collect()
        };
        let sample_image = Image {
            width: w,
            height: h,
            pixels: rand::thread_rng().gen_iter().take(w * h).collect()
        };
        let ref_p = preprocess_image(ref_image);
        let sample_p = preprocess_image(sample_image);
        b.iter(|| {
            align(&ref_p, &sample_p)
        });
    }
}
