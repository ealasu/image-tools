#![feature(fs_read_write)]

extern crate image;
extern crate rawspeed;
extern crate turbojpeg;
extern crate glob;
extern crate ndarray;


#[cfg(test)]
mod tests {
    use std::fs;
    use turbojpeg;
    use rawspeed;
    use image::Image;
    use image::prelude::*;
    use glob::glob;
    use ndarray::prelude::*;

    #[test]
    fn it_works() {
        // 1. read a bias frame, stretch & save as jpeg
        //stretch_raw("data/bias-frames/IMG_1010.CR2", "out/bias.jpg");
        //stretch_raw("test/bias-frame.CR2", "out/bias-frame.jpg", false);
        //stretch_raw("test/dark-frame.CR2", "out/dark-frame.jpg", false);
        //stretch_raw("test/dark-frame.CR2", "out/dark-frame-minus-bias.jpg", true);
        //stretch_raw("test/dark-frame-2.CR2", "out/dark-frame-2.jpg", false);

        let mut stack = None;
        let mut count: usize = 0;
        for entry in glob("/Volumes/data/photos/2017/2017-12-16-darks/101CANON/*.CR2").unwrap() {
            let entry = entry.unwrap();
            println!("{}", entry.display());
            let img = rawspeed::decode(&fs::read(entry).unwrap()).unwrap().scale_to_f32();
            stack = Some(if let Some(mut stack) = stack {
                stack += &img;
                count += 1;
                stack
            } else {
                img
            })
        }
        let mut img = stack.unwrap();
        img /= count as f32;
        println!("minmax: {:?}", img.min_max());
        let img = img.stretch::<u8>(0.0, 0.02, 0, 255).to_rgb();
        fs::write("out/stacked.jpg", turbojpeg::compress(&img).unwrap()).unwrap();
    }

    //fn stack() {
    //}

    //fn stretch_raw(input: &str, output: &str, sub_bias: bool) {
        //let bias = rawspeed::decode(&fs::read("test/bias-frame.CR2").unwrap()).unwrap()
            ////.center_crop(400, 400)
            //.scale_to_f32();
        //let mut img = rawspeed::decode(&fs::read(input).unwrap()).unwrap()
            ////.center_crop(400, 400)
            //.scale_to_f32();
        //if sub_bias {
            //img -= &bias;
        //}
        ////let img = img.stretch_to_bounds::<u8>();
        //let img = img.stretch::<u8>(0.0, 0.1, 0, 255);
        //println!("minmax: {:?}", img.min_max());
        ////println!("{:?}", &img.pixels()[991..999]);
        ////let img = img.invert();
        ////println!("{:?}", &img.pixels()[991..999]);
        //let img = img.to_rgb();
        //fs::write(format!("{}.dat", output), img.as_bytes()).unwrap();
        //fs::write(output, turbojpeg::compress(&img).unwrap()).unwrap();
    //}
}
