extern crate docopt;
extern crate rustc_serialize;
extern crate star_stuff;
extern crate donuts;
extern crate image;

use docopt::Docopt;
use star_stuff::drizzle::ImageStack;
use star_stuff::point::*;
use image::{Image, Rgb};


const USAGE: &'static str = "
Stacker.

Usage:
    stack <output> <input>...
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_output: String,
    arg_input: Vec<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    //stack(args);
    align(args);

}

fn open(path: &str) -> Image<f32> {
    Image::<f32>::open(path).center_crop(900, 900)
}

fn align(args: Args) {
    let ref_image = open(&args.arg_input[0]);
    let reference = donuts::preprocess_image(ref_image.clone());

    for (i, file) in args.arg_input.iter().enumerate() {
        println!("adding {}", i);
        let mut stack = ImageStack::new(ref_image.width, ref_image.height, 1.0, 0.9);
        //let raw_sample = Image::<u16>::open_raw(file).to_rggb();
        let sample_image = open(&file);
        let p = donuts::preprocess_image(sample_image.clone());
        let (x, y) = donuts::align(&reference, &p);
        println!("offset: {},{}", x, y);
        stack.add(&sample_image, Vector { x:x, y:y });
        let mut img = stack.into_image();
        for y in 0..img.height {
            *img.pixel_at_mut(900 / 2, y) *= 0.5;
        }
        for x in 0..img.width {
            *img.pixel_at_mut(x, 900 / 2) *= 0.5;
        }
        img.save(&format!("{}/{}.jpg", args.arg_output, i));
    }
}

fn stack(args: Args) {
    println!("processing ref image");
    let raw_ref = Image::<u16>::open_raw(&args.arg_input[0]);
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

    let mut stack = ImageStack::new(raw_ref.width, raw_ref.height, 2.0, 0.95);

    //let ref_image = raw_ref
        //.to_rggb()
        //.to_green_interpolated()
        //.center_crop(900, 900);
    let ref_image = open(&args.arg_input[0]);
    let reference = donuts::preprocess_image(ref_image);

    println!("stacking");
    for file in args.arg_input.iter() {
        println!("adding {}", file);
        let raw_sample = Image::<u16>::open_raw(file).to_rggb();
        //let sample_image = raw_sample
            //.to_green_interpolated()
            //.center_crop(900, 900);
        let sample_image = open(&file);
        let p = donuts::preprocess_image(sample_image);
        let (x, y) = donuts::align(&reference, &p);
        println!("offset: {},{}", x, y);
        stack.add(&raw_sample, Vector { x:x, y:y });
    }

    let img = stack.into_image();
    img.to_rgb().save(&args.arg_output);
}
