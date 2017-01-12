use image::Image;
use rgb::Rgb;

pub enum ImageKind {
    U8(Image<u8>),
    U16(Image<u16>),
    F32(Image<f32>),
    F64(Image<f64>),
    RgbU8(Image<Rgb<u8>>),
    RgbF32(Image<Rgb<f32>>),
    RgbF64(Image<Rgb<f64>>),
}
