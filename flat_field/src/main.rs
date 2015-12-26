extern crate docopt;
extern crate simple_parallel;
extern crate crossbeam;
extern crate rustc_serialize;
extern crate star_stuff;
extern crate image;

use simple_parallel::Pool;
use docopt::Docopt;
use star_stuff::*;
use image::*;


const USAGE: &'static str = "
field flattener.

Usage:
    flat_field <output> <input>...
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_output: String,
    arg_input: Vec<String>,
}

fn main() {
    /*
      Steps:
      read each raw file, convert to f32, stack them
      adjust the result to get the G frame
        for each component of the bayer matrix,
        compute the average
        res = average / pixel
      save the result to a FITS file
    */

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let mut pool = Pool::new(5);
    let stack = crossbeam::scope(|scope| {
        pool.map(scope, &args.arg_input, |filename| {
            let image: GrayImage<u16> = GrayImage::open_raw(filename);
            image.rescale_to_f32()
        }).fold(None, |stack, img| {
            let stack = if let Some(s) = stack {
                s
            } else {
                ImageStack::new(img.width(), img.height())
            };
            stack.add(img, Default::default());
            Some(stack)
        })
    });
    let img = stack.into_image();

}
