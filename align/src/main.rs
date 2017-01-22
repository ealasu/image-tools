extern crate docopt;
extern crate rustc_serialize;
extern crate donuts;
extern crate image;
extern crate align_api;
extern crate rayon;

use std::fs;
use docopt::Docopt;
use rayon::prelude::*;
use image::Image;
use align_api::AlignedImage;

const USAGE: &'static str = "
Align.

Usage:
    align --output=<filename> <input>...
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_output: String,
    arg_input: Vec<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let ref_image = Image::<f32>::open(&args.arg_input[0]);
    let three_axis = donuts::three_axis_2d::ThreeAxisDonuts::new(&ref_image);

    let res: Vec<_> = args.arg_input
        .into_par_iter()
        .map(|filename| {
            let filename = fs::canonicalize(filename).unwrap();
            println!("aligning {:?}", filename);
            let sample_image = Image::<f32>::open(&filename);
            let transform = three_axis.align(&sample_image);
            println!("offset: {:?}", transform);
            AlignedImage {
                filename: filename.to_string_lossy().into_owned(),
                transform: transform,
            }
        })
        .collect();

    align_api::write(&res, &args.flag_output);
}
