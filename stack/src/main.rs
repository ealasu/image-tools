extern crate docopt;
extern crate rustc_serialize;
extern crate star_stuff;
extern crate donuts;
extern crate image;
extern crate rayon;
extern crate align_api;

use docopt::Docopt;
use star_stuff::drizzle::{self, ImageStack};
use image::{Image, Rgb, RgbBayer, ImageKind};
use rayon::prelude::*;


const USAGE: &'static str = "
Stacker.

Usage:
    stack --output=<filename> --flat=<filename> --alignment=<filename> <input>...
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_output: String,
    flag_flat: String,
    flag_alignment: String,
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
    let factor: f64 = 1.0;
    let raw_ref = Image::<u16>::open_raw(&args.arg_input[0]);
    let w = (raw_ref.width as f64 * factor) as usize;
    let h = (raw_ref.height as f64 * factor) as usize;
    let flat = if let ImageKind::F64(v) = ImageKind::open_fits(&args.flag_flat) {
        v
    } else {
        panic!();
    };
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
    //let ref_image = raw_ref
        //.to_rggb()
        //.to_green_interpolated()
        //.center_crop(900, 900);

    let alignment = align_api::read(&args.flag_alignment);

    let mut stack = Image::<RgbBayer<f64>>::new(w, h);
    for file in args.arg_input.iter() {
        println!("adding {}", file);
        let transform = alignment
            .iter()
            .find(|i| &i.filename == file).expect("missing alignment")
            .transform.to_f64();
        let mut img = Image::<u16>::open_raw(&file).to_f32().to_f64();
        println!("flattening");
        img /= &flat;
        println!("to_rggb");
        let img = img.to_rggb();
        println!("stacking");
        drizzle::add(&mut stack, &img, transform, factor, 0.80);
    }

    //let img = args.arg_input
        //.into_par_iter()
        //.map(|file| {
            //stack
        //})
        //.reduce(
            //|| {
                //println!("reduce init");
                //Image::<RgbBayer<f64>>::new(w, h)
            //},
            //|a, b| {
                //println!("adding");
                //a + b
            //});

    let img = stack;
    img.to_rgb().save_fits(&args.flag_output);
    let holes = img.center_crop(900, 900).holes();
    println!("holes min/max: {:?}", holes.min_max());
    holes.to_u8().save_jpeg_file("holes.jpg");
}

fn open(path: &str) -> Image<f32> {
    Image::<f32>::open(path)
}
