use image::Image;
use rgb::Rgb;
use std::io::BufReader;
use std::fs::File;
use fits;

pub enum ImageKind {
    U8(Image<u8>),
    U16(Image<u16>),
    F32(Image<f32>),
    F64(Image<f64>),
    RgbU8(Image<Rgb<u8>>),
    RgbF32(Image<Rgb<f32>>),
    RgbF64(Image<Rgb<f64>>),
}

impl ImageKind {
    pub fn open_fits(path: &str) -> Self {
        let mut r = BufReader::new(File::open(path).unwrap());
        let (shape, data) = fits::read_image(&mut r);
        assert_eq!(shape.len(), 2);
        let w = shape[0];
        let h = shape[1];
        match data {
            fits::Data::F32(v) => {
                ImageKind::F32(Image {
                    width: w,
                    height: h,
                    pixels: v,
                })
            },
            fits::Data::F64(v) => {
                ImageKind::F64(Image {
                    width: w,
                    height: h,
                    pixels: v,
                })
            },
            fits::Data::U16(v) => {
                ImageKind::U16(Image {
                    width: w,
                    height: h,
                    pixels: v,
                })
            },
        }
    }
}

