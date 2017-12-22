#![feature(fs_read_write)]

extern crate image;
extern crate rawspeed;
extern crate turbojpeg;


#[cfg(test)]
mod tests {
    use std::fs;
    use turbojpeg;
    use rawspeed;
    use image::Image;
    use image::prelude::*;

    #[test]
    fn it_works() {
        // 1. read a bias frame, stretch & save as jpeg
        //stretch_raw("data/bias-frames/IMG_1010.CR2", "out/bias.jpg");
        stretch_raw("test/bias-frame.CR2", "out/bias-frame.jpg", false);
        stretch_raw("test/dark-frame.CR2", "out/dark-frame.jpg", false);
        stretch_raw("test/dark-frame.CR2", "out/dark-frame-minus-bias.jpg", true);
        stretch_raw("test/dark-frame-2.CR2", "out/dark-frame-2.jpg", false);
    }

    fn stretch_raw(input: &str, output: &str, sub_bias: bool) {
        let bias = rawspeed::decode(&fs::read("test/bias-frame.CR2").unwrap()).unwrap()
            //.center_crop(400, 400)
            .scale_to_f32();
        let mut img = rawspeed::decode(&fs::read(input).unwrap()).unwrap()
            //.center_crop(400, 400)
            .scale_to_f32();
        if sub_bias {
            img -= &bias;
        }
        let img = img.stretch_to_bounds::<u8>();
        //let img = img.stretch::<u8>(0.0, 0.01, 0, 255);
        println!("minmax: {:?}", img.min_max());
        //println!("{:?}", &img.pixels()[991..999]);
        //let img = img.invert();
        //println!("{:?}", &img.pixels()[991..999]);
        let img = img.to_rgb();
        fs::write(format!("{}.dat", output), img.as_bytes()).unwrap();
        fs::write(output, turbojpeg::compress(&img).unwrap()).unwrap();
    }
}
