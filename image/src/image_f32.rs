use std::fs::File;
use std::path::Path;
use std::u8;
use std::f32;
use std::u16;
use std::io::{BufReader, BufWriter};
use ::image::*;
//use fits;
use rgb::Rgb;
use rgb_bayer::RgbBayer;
use num::Float;
use num::cast::NumCast;

impl<'a, P: Float + Default> ImageSlice<'a, P> {
    pub fn to_rggb(&self) -> Image<RgbBayer<P>> {
        let mut pixels = Vec::with_capacity(self.dimensions.width * self.dimensions.height);
        for y in 0..self.dimensions.height {
            for x in 0..self.dimensions.width {
                let gray = *self.pixel_at(x, y);
                let x = x % 2;
                let y = y % 2;
                let mut pix: RgbBayer<P> = Default::default();
                {
                    let (v, vc) = match (x, y) {
                        (0, 0) => (&mut pix.r, &mut pix.rc),
                        (1, 0) => (&mut pix.g, &mut pix.gc),
                        (0, 1) => (&mut pix.g, &mut pix.gc),
                        (1, 1) => (&mut pix.b, &mut pix.bc),
                        _ => panic!("invalid bayer coords: {},{}", x, y)
                    };
                    *v = gray;
                    *vc = P::one();
                }
                pixels.push(pix);
            }
        }
        Image {
            dimensions: ImageDimensions {
                width: self.dimensions.width,
                height: self.dimensions.height,
                pitch: self.dimensions.width,
            },
            pixels: pixels,
        }
    }

}

impl<'a> ImageSlice<'a, f32> {

    //pub fn open<P: AsRef<Path>>(path: P) -> Self {
        //let (width, height, data) = convert_open(path, "gray");
        //Image {
            //width: width,
            //height: height,
            //pixels: data,
        //}
    //}

    //pub fn open_fits(path: &str) -> Self {
        //let mut r = BufReader::new(File::open(path).unwrap());
        //let (shape, data) = fits::read_image(&mut r);
        //assert_eq!(shape.len(), 2);
        //let w = shape[0];
        //let h = shape[1];
        //let pixels = match data {
            //fits::Data::F32(v) => v,
            //_ => panic!()
        //};
        //Image {
            //width: w,
            //height: h,
            //pixels: pixels,
        //}
    //}

    //pub fn open_jpeg_file<P: AsRef<Path>>(path: P) -> Self {
        //Image::<Rgb<u8>>::open_jpeg_file(path).to_f32().to_gray()
    //}

    //pub fn save(&self, path: &str) {
        //convert_save(&self.pixels[..], self.width, self.height, "gray", "grayscale", path);
    //}

    //pub fn save_fits(&self, filename: &str) {
        //let mut f = BufWriter::new(File::create(filename).unwrap());
        //let shape = [self.width, self.height];
        //fits::write_image(&mut f, &shape[..], &fits::Data::F32(self.pixels.clone()));
    //}
}

impl Image<f64> {
    //pub fn save_fits(&self, filename: &str) {
        //let mut f = BufWriter::new(File::create(filename).unwrap());
        //let shape = [self.width, self.height];
        //fits::write_image(&mut f, &shape[..], &fits::Data::F64(self.pixels.clone()));
    //}
}
