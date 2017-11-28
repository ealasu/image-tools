extern crate structopt;
#[macro_use] extern crate structopt_derive;
extern crate donuts;
extern crate image;

use structopt::StructOpt;
use image::{Image, Rgb, ImageKind};

#[derive(StructOpt, Debug)]
#[structopt(name = "post", about = "")]
struct Args {
    #[structopt(long = "output")]
    flag_output: String,
    #[structopt(long = "input")]
    arg_input: String,
}

fn main() {
    let args = Args::from_args();
    let img = ImageKind::open_fits(&args.arg_input);
    let img = if let ImageKind::RgbF64(v) = img {
        v
    } else {
        panic!("wrong image type: {:?}", img);
    };

    img
        //.remove_background(0.97)
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
