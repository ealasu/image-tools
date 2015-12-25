extern crate docopt;
extern crate simple_parallel;
extern crate rustc_serialize;
extern crate star_stuff;

use simple_parallel::Pool;
use crossbeam;
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
    crossbeam::scope(|scope| {
        pool.map(scope, &args.arg_input, |filename| {
            let image = GrayImage<u16>::open_raw(filename).rescale_to_f32();
            let channel = &image.channels[0];
            let stars = StarFinder::new(channel);
            let refined_stars = stars.map(|approx_center| {
                refine_star_center(channel, approx_center, aperture)
            }).collect::<Vec<_>>();
            (filename.clone(), refined_stars)
        }).collect()
    });

}
