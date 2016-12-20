//#![feature(loop_break_value)]

#[macro_use] extern crate log;
extern crate crossbeam;
extern crate tempfile;
extern crate log4rs;
extern crate scope_client;
#[cfg(test)] extern crate env_logger;

//mod signal;
mod autoguider;
mod camera;
mod aligner;
mod mount;
mod pos;

//use std::thread;
use std::time::Duration;
use camera::Camera;
use aligner::Aligner;
use mount::Mount;
use pos::Vector;

const MAX: f32 = 100.0;

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let camera = Camera::new();
    let mut aligner = Aligner::new();
    let mut mount = Mount::new();
    let num_images = 150;
    let mut range = 0..num_images;
    let shot_duration = Duration::from_secs(18);

    autoguider::run_autoguider(
        0..num_images,
        shot_duration,
        Vector { x: 0.0, y: 0.0 },
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
            offset
        },
        |amount| {
            if amount.x.abs() > MAX || amount.y.abs() > MAX {
                warn!("not slewing because too big: {:?}", amount);
                return;
            }
            info!("slewing mount to offset: {:?}", amount);
            mount.slew(amount);
        }
    );
}
