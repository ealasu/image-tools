extern crate docopt;
extern crate simple_parallel;
extern crate rustc_serialize;
extern crate star_stuff;

use simple_parallel::Pool;
use docopt::Docopt;
use star_stuff::*;


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

    let mut pool = Pool::new(4);

    println!("finding stars");
    let res = find_stars(&mut pool, args.arg_input);
    for (ref f, ref v) in res.iter() {
        println!("found {} stars in {:?}", v.len(), f);
        for star in v.iter() {
            println!("{},{}", star.x, star.y);
        }
    }
    // TODO: eliminate images with distorted stars

    println!("aligning");
    let res = align_images(&mut pool, res);
    println!("aligned:");
    for img in res.iter() {
        println!("{:?}", img);
    }

    println!("stacking");
    stack(&res, &args.arg_output);
}
