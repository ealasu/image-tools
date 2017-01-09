extern crate docopt;
extern crate rustc_serialize;
extern crate star_stuff;
extern crate donuts;
extern crate image;
extern crate rayon;

use docopt::Docopt;
use star_stuff::drizzle::{self, ImageStack};
use image::{Image, Rgb, RgbBayer};
use rayon::prelude::*;


const USAGE: &'static str = "
Stacker.

Usage:
    stack --output=<filename> --flat=<filename> <input>...
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_output: String,
    flag_flat: String,
    arg_input: Vec<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    stack(args);
    //align(args);

}


//fn align(args: Args) {
//    let ref_image = open(&args.arg_input[0]);
//    let reference = donuts::preprocess_image(ref_image..center_crop(900, 900));
//
//    for (i, file) in args.arg_input.iter().enumerate() {
//        println!("adding {}", i);
//        let mut stack = ImageStack::new(ref_image.width, ref_image.height, 1.0, 0.9);
//        //let raw_sample = Image::<u16>::open_raw(file).to_rggb();
//        let sample_image = open(&file);
//        let p = donuts::preprocess_image(sample_image.clone());
//        let (x, y) = donuts::align(&reference, &p);
//        println!("offset: {},{}", x, y);
//        stack.add(&sample_image, Vector { x:x, y:y });
//        let mut img = stack.finish();
//        for y in 0..img.height {
//            *img.pixel_at_mut(900 / 2, y) *= 0.5;
//        }
//        for x in 0..img.width {
//            *img.pixel_at_mut(x, 900 / 2) *= 0.5;
//        }
//        img.save(&format!("{}/{}.jpg", args.flag_output, i));
//    }
//}

fn stack(args: Args) {
    println!("processing ref image");
    let factor = 1.0;
    let raw_ref = Image::<u16>::open_raw(&args.arg_input[0]);
    let w = (raw_ref.width as f32 * factor) as usize;
    let h = (raw_ref.height as f32 * factor) as usize;
    let flat = Image::<f32>::open_fits(&args.flag_flat);
    //for v in raw_ref.to_f32().pixels.iter() {
        //println!("{}", v);
    //}
    //raw_ref
        //.to_f32()
        //.center_crop(900, 900)
        //.save("ref-bayer.tif");
    //raw_ref
        //.to_rggb()
        //.to_rgb()
        //.center_crop(900, 900)
        //.save("ref-bayer-rgb.tif");
    //raw_ref
        //.to_rggb()
        //.to_green_interpolated()
        //.center_crop(900, 900)
        //.save("ref-bayer-green-inter.tif");
    //let ref_image = raw_ref
        //.to_rggb()
        //.to_green()
        //.center_crop(900, 900);
    //ref_image.save("ref.tif");

    //let mut stack = ImageStack::new(raw_ref.width, raw_ref.height, 2.0, 0.80);

    //let ref_image = raw_ref
        //.to_rggb()
        //.to_green_interpolated()
        //.center_crop(900, 900);
    let ref_image = open(&args.arg_input[0]);
    let reference = donuts::preprocess_image(ref_image.center_crop(900, 900));
    let three_axis = donuts::three_axis::ThreeAxisDonuts::new(&ref_image);

    println!("stacking");

    let img = args.arg_input
        .into_par_iter()
        .map(|file| {
            println!("adding {}", file);
            let sample_image = open(&file);
            three_axis.align(&sample_image);
            let p = donuts::preprocess_image(sample_image.center_crop(900, 900));
            let d = donuts::align(&reference, &p);
            println!("offset: {:?}", d);
            let img = Image::<u16>::open_raw(&file).to_f32();
            let img = img / &flat;
            let img = img.to_rggb();
            let mut stack = Image::<RgbBayer>::new(w, h);
            drizzle::add(&mut stack, &img, d, factor, 0.80);
            stack
        })
        .reduce(
            || {
                println!("reduce init");
                Image::<RgbBayer>::new(w, h)
            },
            |a, b| {
                println!("adding");
                a + b
            });

    //for file in args.arg_input.iter() {
        //println!("adding {}", file);
        //let sample_image = open(&file);
        //let p = donuts::preprocess_image(sample_image);
        //let (x, y) = donuts::align(&reference, &p);
        //println!("offset: {},{}", x, y);

        //let raw_sample = Image::<u16>::open_raw(file).to_f32() / &flat;
        //stack.add(&raw_sample.to_rggb(), Vector { x:x, y:y });
    //}
    //let img = stack.into_image();
    img.to_rgb().save(&args.flag_output);
    let holes = img.center_crop(900, 900).holes();
    println!("holes min/max: {:?}", holes.min_max());
    holes.to_u8().save_jpeg_file("holes.jpg");
}

fn open(path: &str) -> Image<f32> {
    Image::<f32>::open(path)
}
