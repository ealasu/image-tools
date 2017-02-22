extern crate mount_service_api;
extern crate astrometry;
extern crate gphoto;
extern crate regex;
#[macro_use] extern crate log;
extern crate retry;
#[macro_use] extern crate error_chain;

pub mod errors;

use std::fs;
use std::thread;
use std::time::Duration;
use mount_service_api::{Client, Pos};
use regex::Regex;
use retry::retry;
use errors::*;


fn shoot_and_solve() -> Result<(f64, f64)> {
  info!("shooting...");
  let img = gphoto::shoot(gphoto::Options {
    keep_raw: false,
    iso: "20".into(), // 6400
    shutter_speed: "6".into(),
  })?;
  fs::copy(img.path(), "/mnt/ramdisk/latest.jpg").unwrap();
  info!("solving...");
  let img_path = &img.path().to_str().unwrap();
  Ok(astrometry::solve(img_path)?)
}

pub fn point(client: &Client, ra: &str, dec: &str, threshold: f64) {
  client.start().unwrap().unwrap();
  thread::sleep(Duration::from_secs(2));

  let desired_ra = read_ra(ra).expect("failed to parse ra");
  let desired_dec = read_ra(dec).expect("failed to parse dec");

  loop {
    let (ra, dec) = retry(
      5, 2000,
      || {
        shoot_and_solve()
      },
      |res| {
        if res.is_err() {
          error!("shoot_and_solve failed, trying again: {:?}", res);
        }
        res.is_ok()
      }
    ).unwrap().unwrap();
    let d_ra = desired_ra - ra;
    let d_dec = desired_dec - dec;
    info!("d_ra: {} d_dec: {}", d_ra, d_dec);
    if d_ra.abs() < threshold && d_dec.abs() < threshold {
      break;
    }
    info!("slewing...");
    client.slew_by(Pos { ra: d_ra, dec: d_dec }).unwrap().unwrap();

    info!("waiting for slew to end...");
    thread::sleep(Duration::from_secs(1));
    while client.is_slewing().unwrap().unwrap() {
      thread::sleep(Duration::from_secs(1));
    }
    thread::sleep(Duration::from_secs(2));
  }

  info!("resetting position...");
  client.reset_position(Pos {
    ra: desired_ra,
    dec: desired_dec,
  }).unwrap();
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
