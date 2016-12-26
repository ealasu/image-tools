#[macro_use] extern crate log;
extern crate crossbeam;
extern crate tempfile;
extern crate pid_control;
extern crate log4rs;
extern crate mount_service_api;
#[cfg(test)] extern crate env_logger;

mod autoguider;
mod camera;
mod aligner;
mod mount;
mod pos;

use std::time::Duration;
use camera::Camera;
use aligner::Aligner;
use mount::Mount;
use pos::*;
use pid_control::{Controller, PIDController};

const MAX: f64 = 150.0;

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let camera = Camera::new();
    let mut aligner = Aligner::new();
    let mut mount = Mount::new();
    let num_images = 150;
    let mut range = 0..num_images;
    let shot_duration = Duration::from_secs(5 + 15);
    let mut ra_controller = PIDController::new(0.3, 0.04, 0.0);
    let mut dec_controller = PIDController::new(0.1, 0.03, 0.0);

    autoguider::run_autoguider(
        0..num_images,
        shot_duration,
        Default::default(),
        |id| {
            info!("shooting image {}", id);
            let image = camera.shoot();
            info!("finished shooting image {}", id);
            image
        },
        |image| {
            info!("calculating offset");
            let offset = aligner.align(image);
            info!("calculated offset: {:?}", offset);
            if offset.x.abs() > MAX || offset.y.abs() > MAX {
                warn!("not slewing because too big: {:?}", offset);
                None
            } else {
                Some(RaDec {
                    ra: -ra_controller.update(offset.y, 1.0), // TODO: calc time delta
                    dec: -dec_controller.update(offset.x, 1.0),
                })
            }
        },
        |speed| {
            info!("setting slew speed: ra: {}, dec: {}", speed.ra, speed.dec);
            mount.slew(pixel_to_step(speed.ra), pixel_to_step(speed.dec));
        }
    );
}

fn pixel_to_step(v: f64) -> i32 {
    let pixel_size_um = 6.54; // for Canon 6D
    let focal_length_mm = 200.0;
    let pixel_size_arcsec = pixel_size_um / focal_length_mm * 206.3;
    let v = v * pixel_size_arcsec;
    let secs_per_step = 69044.0 / 1000000.0;
    let arcsec_per_sec = (360.0*60.0*60.0) / (23.9344699*60.0*60.0);
    let res: f64 = v / arcsec_per_sec / secs_per_step;
    res as i32
}
