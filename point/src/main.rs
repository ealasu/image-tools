extern crate mount_service_api;
extern crate docopt;
extern crate rustc_serialize;
extern crate astrometry;
extern crate tempfile;

mod gphoto;

use std::fs::File;
use std::thread;
use std::time::Duration;
use docopt::Docopt;
use mount_service_api::{Client, Msg, Pos};

const USAGE: &'static str = "
Usage:
  slew --ra=<degrees> --dec=<degrees> --threshold=<degrees>
";

#[derive(RustcDecodable)]
struct Args {
    flag_ra: f64,
    flag_dec: f64,
    flag_threshold: f64,
}

fn shoot_and_solve() -> (f64, f64) {
  let img = gphoto::shoot();
  astrometry::solve(&img.path().to_str().unwrap())
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    let client = Client::new("ubuntu:1234").unwrap();

    loop {
      let (ra, dec) = shoot_and_solve();
      let d_ra = args.flag_ra - ra;
      let d_dec = args.flag_dec - dec;
      println!("d_ra: {} d_dec: {}", d_ra, d_dec);
      if d_ra.abs() < args.flag_threshold && d_dec.abs() < args.flag_threshold {
        break;
      }
      client.slew_by(Pos { ra: d_ra, dec: d_dec });
      // TODO: either change slew_by to wait for slew to finish, or estimate how long it takes and
      // sleep
      thread::sleep(Duration::from_secs(5));
    }
}
