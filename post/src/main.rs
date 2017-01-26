extern crate docopt;
extern crate rustc_serialize;
extern crate donuts;
extern crate image;

use docopt::Docopt;
use image::{Image, Rgb, ImageKind};

const USAGE: &'static str = "
Post.

Usage:
    post --output=<filename> <input>
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_output: String,
    arg_input: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let img = ImageKind::open_fits(&args.arg_input);
    let img = if let ImageKind::RgbF64(v) = img {
        v
    } else {
        panic!("wrong image type: {:?}", img);
    };

    img
        .remove_background(0.97)
        //.map(|&p| {
            ////(p * 9.0).truncate(0.0, 1.0)
            //p * Rgb {
                //r: 0.5,
                //g: 1.0,
                //b: 1.0,
            //}
        //})
        .gamma(1.0 / 2.2)
        .stretch(0.0, 1.0)
        .to_f32()
        .save(&args.flag_output);
        //.to_u8()
        //.save_jpeg_file(&args.flag_output);
}
