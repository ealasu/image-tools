#![feature(fs_read_write)]

extern crate image;
extern crate rawspeed;
extern crate turbojpeg;


#[cfg(test)]
mod tests {
    use std::fs;
    use rawspeed::RawImage;
    use turbojpeg;
    use image::Image;

    #[test]
    fn it_works() {
        // 1. read a bias frame, stretch & save as jpeg
        //stretch_raw("data/bias-frames/IMG_1010.CR2", "out/bias.jpg");
        stretch_raw("data/dark-frames/IMG_0506.CR2", "out/dark.jpg");
    }

    fn stretch_raw(input: &str, output: &str) {
        let raw_data = fs::read(input).unwrap();
        let img = RawImage::decode(&raw_data).unwrap();
        let img = img.scale_to_f32();
        let img = img.stretch_to_bounds::<u8>();
        println!("minmax: {:?}", img.scale_to_f32().min_max());
        println!("{:?}", &img.pixels()[991..999]);
        let img = img.to_rgb();
        fs::write("out/dat", img.as_bytes()).unwrap();
        fs::write(output, turbojpeg::compress(&img)).unwrap();
    }
}
