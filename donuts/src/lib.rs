#![feature(test)]
#![feature(iter_min_by)]
#![feature(iter_max_by)]

#[macro_use] extern crate log;
extern crate quickersort;
extern crate simd;
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
pub mod three_axis_2d;
pub mod cross_range;
pub mod correlation_2d;

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
        //let open = |file: &str| {
            //Image::<Rgb<u8>>::open_jpeg_file(file).to_f32()
                //.to_gray()
                //.center_crop(900, 900)
        //};

        let SIZE: usize = 900;
        let MARGIN: usize = 300;
        let d_c = Vector { x: -4.0f32, y: -2.0f32 };

        let ref_img = Image::<Rgb<u8>>::open_jpeg_file("test/a.jpg").to_f32();
        let ref_image = ref_img
            .crop(
                ref_img.width - SIZE - MARGIN,
                MARGIN,
                SIZE, SIZE)
            .remove_background(1.0)
            .to_gray();

        //let sample_image = ref_image.clone();
        let sam_img = Image::<Rgb<u8>>::open_jpeg_file("test/b.jpg").to_f32();
        let sample_image = sam_img
            .crop(
                //3977, 303,
                (sam_img.width as isize - SIZE as isize - MARGIN as isize - d_c.x as isize) as usize,
                (MARGIN as isize - d_c.y as isize) as usize,
                SIZE, SIZE)
            .remove_background(1.0)
            .to_gray();

        let n = 300;

        println!("preprocessing");
        let ref_p = preprocess_image(ref_image.clone());
        //ref_image.save("test/ref-gray-bg.jpg");
        save_array(&ref_p.x, "test/ref-p-x");
        save_array(&ref_p.y, "test/ref-p-y");
        let sample_p = preprocess_image(sample_image.clone());
        //sample_image.save("test/sample-gray-bg.jpg");
        save_array(&sample_p.x, "test/sample-p-x");
        save_array(&sample_p.y, "test/sample-p-y");
        println!("aligning");
        save_array(&correlation::correlation(&ref_p.x, &sample_p.x, n), "test/corr-x");
        save_array(&correlation::correlation(&ref_p.y, &sample_p.y, n), "test/corr-y");
        let offset = align(&ref_p, &sample_p, n);

        let corr_2d = correlation_2d::correlation_2d(&ref_image, &sample_image, 50);
        let corr_2d_peak = correlation_2d::correlation_peak(&corr_2d);
        corr_2d.save("test/corr_2d.jpg");
        println!("2d corr peak: {:?}", corr_2d_peak);

        assert_eq!(offset, Vector { x: -15.721349, y: -18.200153 });
    }

    #[test]
    fn test_bad() {
        let SIZE = 1200;
        let MARGIN = 300;
        let d_c = Vector { x: -4.0, y: -2.0 };

        let ref_img = Image::<Rgb<u8>>::open_jpeg_file("test/a.jpg").to_f32();
        let ref_img = ref_img
            .crop(
                ref_img.width - SIZE - MARGIN,
                MARGIN,
                SIZE, SIZE)
            .remove_background(1.0)
            .to_gray();

        let sam_img = Image::<Rgb<u8>>::open_jpeg_file("test/b.jpg").to_f32();
        let sam_img = sam_img
            .crop(
                3977, 303,
                //sam_img.width - SIZE - MARGIN - d_c.x as usize,
                //MARGIN - d_c.y as usize,
                SIZE, SIZE)
            .remove_background(1.0)
            .to_gray();

        let ref_p = preprocess_image(ref_img);
        let sam_p = preprocess_image(sam_img);
        let offset = align(&ref_p, &sam_p, 300);
        assert_eq!(offset, Vector { x: 0.0, y: 0.0 });
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
