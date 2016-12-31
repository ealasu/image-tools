extern crate docopt;
extern crate rustc_serialize;
extern crate star_stuff;
extern crate donuts;
extern crate image;

use docopt::Docopt;
use star_stuff::ImageStack;
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

    println!("aligning");
    let ref_image = Image::<Rgb<f32>>::open(&args.arg_input[0]).center_crop(900, 900).to_gray();
    let mut stack = ImageStack::new(ref_image.width, ref_image.height);
    let reference = donuts::preprocess_image(ref_image);
    for file in args.arg_input.iter() {
        let img = Image::<Rgb<f32>>::open(file);
        let p = donuts::preprocess_image(img.center_crop(900, 900).to_gray());
        let (x, y) = donuts::align(&reference, &p);
        stack.add(&img, Vector { x:x, y:y });
    }
    stack.into_image().save(&args.arg_output);
}
