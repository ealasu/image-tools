extern crate docopt;
extern crate rustc_serialize;
extern crate donuts;
extern crate star_aligner;
extern crate image;
extern crate align_api;
extern crate rayon;
extern crate tempfile;

use std::fs;
use std::path::Path;
use std::process::Command;
use docopt::Docopt;
use rayon::prelude::*;
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

fn with_fits<F,R,P>(src: P, mut f: F) -> R
where P: AsRef<Path>, F: FnMut(&Path) -> R {
    let out = tempfile::NamedTempFileOptions::new().suffix(".fits").create().unwrap();
    let status = Command::new("convert")
        .arg(src.as_ref())
        .arg("-colorspace")
        .arg("gray")
        .arg(out.path())
        .status()
        .expect("failed to run convert");
    assert!(status.success());
    f(out.path())
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    //let ref_image = Image::<f32>::open(&args.arg_input[0]);
    //let three_axis = donuts::three_axis_2d::ThreeAxisDonuts::new(&ref_image);
    let reference = with_fits(&args.arg_input[0], |filename| {
        star_aligner::Reference::from_image(filename)
    });

    let res: Vec<_> = args.arg_input
        .into_par_iter()
        .filter_map(|filename| {
            let filename = fs::canonicalize(filename).unwrap();
            println!("aligning {:?}", filename);
            //let sample_image = Image::<f32>::open(&filename);
            //let transform = three_axis.align(&sample_image);
            let transform = with_fits(&filename, |fits_filename| {
                reference.align_image(fits_filename)
            });
            if let Some(transform) = transform {
                Some(AlignedImage {
                    filename: filename.to_string_lossy().into_owned(),
                    transform: transform
                })
            } else {
                println!("failed to align {}", filename.to_str().unwrap());
                None
            }
        })
        .collect();

    align_api::write(&res, &args.flag_output);
}
