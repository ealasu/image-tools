extern crate docopt;
extern crate rustc_serialize;
extern crate star_stuff;

use docopt::Docopt;
use star_stuff::*;
use star_stuff::point::*;


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
    //let res = align_images(res);
    let reference = &args.arg_input[0];
    let res: Vec<_> = args.arg_input
        .iter()
        .map(|f| (f.to_string(), align_ext(&reference, &f)))
        .collect();

    println!("aligned:");
    for img in res.iter() {
        println!("{:?}", img);
    }

    println!("stacking");
    stack(&res, &args.arg_output);
}

fn align_ext(reference: &str, image: &str) -> Vector {
    use std::process::Command;
    use std::str;

    let output = Command::new("/Users/emi/repos/projects/donuts-test/main.py")
                         .arg(reference)
                         .arg(image)
                         .output()
                         .expect("failed to execute process");
    let s = str::from_utf8(&output.stdout).unwrap();
    println!("{}", s);
    let mut s = s.split(",");
    let x = s.next().unwrap();
    println!("{}", x);
    let x = x.trim().parse::<f32>().unwrap();
    let y = s.next().unwrap();
    println!("{}", y);
    let y = y.trim().parse::<f32>().unwrap();
    Vector { x: x, y: -y }
    //Vector { x: 0.0, y: 0.0 }
}
