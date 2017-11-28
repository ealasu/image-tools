#[macro_use] extern crate log;
extern crate structopt;
#[macro_use] extern crate structopt_derive;
extern crate crossbeam;
extern crate tempfile;
extern crate pid_control;
extern crate log4rs;
extern crate mount_service_api;
extern crate image;
extern crate donuts;
extern crate geom;
extern crate point;
extern crate gphoto;
extern crate retry;
#[cfg(test)] extern crate env_logger;

mod autoguider;
mod camera;
mod aligner;
mod mount;
mod pos;

use std::time::Duration;
use std::thread;
use std::sync::Mutex;
use camera::Camera;
use aligner::Aligner;
use mount::Mount;
use pos::*;
use pid_control::{Controller, PIDController};
use structopt::StructOpt;
use mount_service_api::Client;
use retry::retry;

const MAX: f32 = 150.0;

#[derive(StructOpt, Debug)]
#[structopt(name = "autoguider", about = "")]
struct Args {
    #[structopt(long = "count", help = "number of images to shoot")]
    flag_count: usize,
    #[structopt(long = "ra")]
    flag_ra: Option<String>,
    #[structopt(long = "dec")]
    flag_dec: Option<String>,
    #[structopt(long = "threshold", help = "threshold in degrees")]
    flag_threshold: Option<f64>,
}

fn main() {
    let args = Args::from_args();
    log4rs::init_file("log4rs.yml", Default::default()).expect("failed to init log4rs");

    let client = Client::new("localhost:1234").expect("failed to create client");
    client.start().unwrap().unwrap();
    thread::sleep(Duration::from_secs(1));

    if let (Some(ra), Some(dec), Some(threshold)) = (args.flag_ra, args.flag_dec, args.flag_threshold) {
        point::point(&client, &ra, &dec, threshold);
    }
    info!("waiting a bit...");
    thread::sleep(Duration::from_secs(5));

    let camera = Mutex::new(Camera::new());
    let mut aligner = Aligner::new();
    let mut mount = Mount::new(client);
    //let shot_duration = Duration::from_secs(5 + 30);
    let shot_duration = Duration::from_secs(3 + 30);
    let mut ra_controller = PIDController::new(0.3, 0.03, 0.0);
    let mut dec_controller = PIDController::new(0.10, 0.015, 0.0);

    mount.start();

    autoguider::run_autoguider(
        0..args.flag_count,
        shot_duration,
        Default::default(),
        |id| {
            info!("shooting image {}", id);
            let camera = camera.lock().unwrap();
            let image = retry(
                4, 2000,
                || {
                    camera.shoot()
                },
                |res| {
                    if res.is_err() {
                        error!("shoot failed, trying again: {:?}", res);
                    }
                    res.is_ok()
                }
            ).unwrap().unwrap();
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
                    ra: -ra_controller.update(-offset.y as f64, 1.0), // TODO: calc time delta
                    dec: -dec_controller.update(offset.x as f64, 1.0),
                })
            }
        },
        |speed| {
            info!("setting slew speed: ra: {}, dec: {}", speed.ra, speed.dec);
            mount.slew(pixel_to_step(speed.ra), pixel_to_step(speed.dec));
        }
    );

    mount.stop();
    println!("Done.");
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
