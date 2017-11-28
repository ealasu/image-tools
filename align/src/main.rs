extern crate structopt;
#[macro_use] extern crate structopt_derive;
extern crate donuts;
extern crate star_aligner;
extern crate image;
extern crate align_api;
extern crate rayon;
extern crate tempfile;
#[macro_use] extern crate log;
extern crate env_logger;

use std::fs;
use std::env;
use std::path::Path;
use std::process::Command;
use structopt::StructOpt;
use rayon::prelude::*;
use align_api::AlignedImage;

#[derive(StructOpt, Debug)]
#[structopt(name = "align", about = "")]
struct Args {
    #[structopt(long = "output", help = "filename")]
    flag_output: String,
    #[structopt(long = "max-stars")]
    flag_max_stars: usize,
    #[structopt(long = "min-matching-stars")]
    flag_min_matching_stars: usize,
    #[structopt(long = "threshold", help = "px")]
    flag_threshold: f64,
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
    let args = Args::from_args();
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init().unwrap();

    info!("aligning {} images", args.arg_input.len());

    //let ref_image = Image::<f32>::open(&args.arg_input[0]);
    //let three_axis = donuts::three_axis_2d::ThreeAxisDonuts::new(&ref_image);
    let reference = with_fits(&args.arg_input[0], |filename| {
        star_aligner::Reference::from_image(filename, star_aligner::Options {
            max_stars: args.flag_max_stars,
            min_matching_stars: args.flag_min_matching_stars,
            threshold: args.flag_threshold,
        })
    });

    let res: Vec<_> = args.arg_input
        .par_iter()
        .filter_map(|filename| {
            let filename = fs::canonicalize(filename).unwrap();
            info!("aligning {:?}", filename);
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
                error!("failed to align {}", filename.to_str().unwrap());
                None
            }
        })
        .collect();

    info!("good: {}, bad: {}", res.len(), args.arg_input.len() - res.len());

    align_api::write(&res, &args.flag_output);
}
