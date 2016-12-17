#![feature(loop_break_value)]

#[macro_use] extern crate log;
extern crate crossbeam;
extern crate tempfile;
extern crate log4rs;

mod signal;
mod autoguider;
mod camera;
mod aligner;
mod mount;
mod pos;

use camera::Camera;
use aligner::Aligner;
use mount::Mount;


fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let camera = Camera::new();
    let mut aligner = Aligner::new();
    let mut mount = Mount::new();

    autoguider::run_autoguider(
        || {
            // shoot an image
            Some(camera.shoot())
        },
        |image| {
            // calculate offset
            aligner.align(image)
        },
        |pos| {
            // slew mount to correct
            mount.slew(pos);
        }
    );
}
