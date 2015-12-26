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

fn bayer_average(channel: &Channel<f32>, offset_x: usize, offset_y: usize) -> f32 {
    let bayer_w = channel.width / 2;
    let bayer_h = channel.height / 2;
    
    let mut sum = 0.0;
    for y in 0..bayer_h {
        for x in 0..bayer_w {
            sum += channel.at(x * 2 + offset_x, y * 2 + offset_y);
        }
    }
    sum / ((bayer_w * bayer_h) as f32)
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
            let mut stack = if let Some(s) = stack {
                s
            } else {
                ImageStack::new(img.width(), img.height())
            };
            stack.add(&img, Default::default());
            Some(stack)
        }).unwrap()
    });
    let mut img = stack.into_image();
    let chan = &mut img.0;

    let avg_r = bayer_average(chan, 0, 0);
    let avg_g = (bayer_average(chan, 1, 0) + bayer_average(chan, 0, 1)) / 2.0;
    let avg_b = bayer_average(chan, 1, 1);

    for y in 0..chan.height {
        for x in 0..chan.width {
            let pixel = chan.at_mut(x, y);
            let bayer_x = x % 2;
            let bayer_y = y % 2;
            let avg = if bayer_x == 0 && bayer_y == 0 {
                avg_r
            } else if bayer_x == 1 && bayer_y == 1 {
                avg_b
            } else {
                avg_g
            };
            *pixel = avg / *pixel;
        }
    }

    img.save(args.arg_output);
}
