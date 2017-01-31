extern crate mount_service_api;
extern crate docopt;
extern crate rustc_serialize;
extern crate astrometry;
extern crate tempfile;
extern crate regex;

mod gphoto;

use std::fs;
use std::thread;
use std::time::Duration;
use docopt::Docopt;
use mount_service_api::{Client, Pos};
use regex::Regex;

const USAGE: &'static str = "
Usage:
  slew --ra=<degrees> --dec=<degrees> --threshold=<degrees>
";

#[derive(RustcDecodable)]
struct Args {
    flag_ra: String,
    flag_dec: String,
    flag_threshold: f64,
}

fn shoot_and_solve() -> (f64, f64) {
  println!("shooting...");
  let img = gphoto::shoot();
  fs::copy(img.path(), "/mnt/ramdisk/latest.jpg").unwrap();
  println!("solving...");
  astrometry::solve(&img.path().to_str().unwrap())
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    let client = Client::new("ubuntu:1234").unwrap();
    client.start().unwrap().unwrap();

    let desired_ra = read_ra(&args.flag_ra).expect("failed to parse ra");
    let desired_dec = read_ra(&args.flag_dec).expect("failed to parse dec");

    loop {
      let (ra, dec) = shoot_and_solve();
      let d_ra = desired_ra - ra;
      let d_dec = desired_dec - dec;
      println!("d_ra: {} d_dec: {}", d_ra, d_dec);
      if d_ra.abs() < args.flag_threshold && d_dec.abs() < args.flag_threshold {
        break;
      }
      println!("slewing...");
      client.slew_by(Pos { ra: d_ra, dec: d_dec }).unwrap().unwrap();

      println!("waiting for slew to end...");
      thread::sleep(Duration::from_secs(1));
      while client.is_slewing().unwrap().unwrap() {
        thread::sleep(Duration::from_secs(1));
      }
      thread::sleep(Duration::from_secs(2));
    }

    println!("resetting position...");
    client.reset_position(Pos {
      ra: desired_ra,
      dec: desired_dec,
    }).unwrap();

    println!("done.");
}

/// Convert RA to degrees
pub fn read_ra(text: &str) -> Option<f64> {
  let parsed = text.parse::<f64>();
  if let Ok(num) = parsed {
    return Some(num)
  }

  let re = Regex::new(r"^([0-9]+)h ([0-9\.]+)m$").unwrap();
  if let Some(captures) = re.captures(text) {
    let h = captures.get(1).unwrap().as_str();
    let h = h.parse::<f64>().unwrap();
    let m = captures.get(2).unwrap().as_str();
    let m = m.parse::<f64>().unwrap();
    return Some(h * 15.0 + m / 4.0);
  }

  let re = Regex::new(r"^([0-9+\-]+)d ([0-9\.]+)m$").unwrap();
  if let Some(captures) = re.captures(text) {
    let d = captures.get(1).unwrap().as_str();
    let d = d.parse::<f64>().unwrap();
    let m = captures.get(2).unwrap().as_str();
    let m = m.parse::<f64>().unwrap();
    let m = if d > 0.0 { m } else { -m };
    return Some(d + m / 60.0);
  }

  None
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_read_ra() {
    assert_eq!(read_ra("123.4"), Some(123.4));
    assert_eq!(read_ra("123.4h"), None);
    assert_eq!(read_ra(""), None);
    assert_eq!(read_ra("4h 14.3m"), Some(63.575));
    assert_eq!(read_ra("-123d 30m"), Some(-123.5));
    assert_eq!(read_ra("+123d 30m"), Some(123.5));
  }
}
