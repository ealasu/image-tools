#![feature(test)]
#![feature(iter_min_by)]
#![feature(iter_max_by)]

#[macro_use] extern crate log;
extern crate quickersort;
extern crate image;
extern crate geom;
#[cfg(test)] extern crate test;
#[cfg(test)] extern crate rand;
#[cfg(test)] extern crate byteorder;

pub mod remove_background;
pub mod projection;
pub mod correlation;
pub mod align;
pub mod three_axis;
pub mod cross_range;

pub use align::align;

use image::Image;
use projection::Projection;


pub fn preprocess_image(mut image: Image<f32>) -> Projection {
    remove_background::remove_background(&mut image, 32);
    Projection::new(&image)
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use rand::{self, Rng};
    use image::{Image, Rgb};
    use geom::Vector;

    fn save_array(data: &[f32], path: &str) {
        use byteorder::{WriteBytesExt, LittleEndian};
        use std::fs::File;
        let mut f = File::create(path).unwrap();
        for v in data.iter() {
            f.write_f32::<LittleEndian>(*v).unwrap();
        }
    }

    #[test]
    fn test_end_to_end() {
        println!("reading ref");
        let ref_image = Image::<Rgb<u8>>::open_jpeg_file("test/ref.jpg").to_f32()
            .to_gray()
            .center_crop(900, 900);
        //ref_image.save("test/ref-gray.jpg");
        println!("reading sample");
        let sample_image = Image::<Rgb<u8>>::open_jpeg_file("test/sample.jpg").to_f32()
            .to_gray()
            .center_crop(900, 900);
        //sample_image.save("test/sample-gray.jpg");
        println!("preprocessing");
        let ref_p = preprocess_image(ref_image);
        //ref_image.save("test/ref-gray-bg.jpg");
        save_array(&ref_p.x, "test/ref-p-x");
        save_array(&ref_p.y, "test/ref-p-y");
        let sample_p = preprocess_image(sample_image);
        //sample_image.save("test/sample-gray-bg.jpg");
        save_array(&sample_p.x, "test/sample-p-x");
        save_array(&sample_p.y, "test/sample-p-y");
        println!("aligning");
        save_array(&correlation::correlation(&ref_p.x, &sample_p.x, 200), "test/corr-x");
        save_array(&correlation::correlation(&ref_p.y, &sample_p.y, 200), "test/corr-y");
        let offset = align(&ref_p, &sample_p, 100);
        assert_eq!(offset, Vector { x: -15.721349, y: -18.200153 });
    }

    #[bench]
    fn bench_preprocess(b: &mut Bencher) {
        let (w, h) = (900, 900);
        let image = Image::<f32>::random(w, h);
        b.iter(|| {
            let image = image.clone();
            preprocess_image(image)
        });
    }

    #[bench]
    fn bench_align(b: &mut Bencher) {
        let (w, h) = (900, 900);
        let ref_image = Image::<f32>::random(w, h);
        let sample_image = Image::<f32>::random(w, h);
        let ref_p = preprocess_image(ref_image);
        let sample_p = preprocess_image(sample_image);
        b.iter(|| {
            align(&ref_p, &sample_p, 100)
        });
    }
}
