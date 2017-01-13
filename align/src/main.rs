extern crate docopt;
extern crate rustc_serialize;
extern crate donuts;
extern crate image;
extern crate align_api;
extern crate rayon;

use std::fs::File;
use std::io::prelude::*;
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

    let ref_image = open(&args.arg_input[0]);
    let three_axis = donuts::three_axis::ThreeAxisDonuts::new(&ref_image);

    let res: Vec<_> = args.arg_input
        .into_par_iter()
        .map(|filename| {
            println!("aligning {}", filename);
            let sample_image = open(&filename);
            let transform = three_axis.align(&sample_image);
            println!("offset: {:?}", transform);
            AlignedImage {
                filename: filename.to_string(),
                transform: transform,
            }
        })
        .collect();

    align_api::write(&res, &args.flag_output);
}

fn open(path: &str) -> Image<f32> {
    Image::<f32>::open(path)
}
