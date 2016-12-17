#![feature(loop_break_value)]
#![feature(receiver_try_iter)]

#[macro_use] extern crate log;
extern crate env_logger;
extern crate crossbeam;
extern crate tempfile;

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
    env_logger::init().unwrap();

    let mut camera = Camera::new();
    let mut aligner = Aligner::new();
    let mut mount = Mount::new();

    autoguider::run_autoguider(
        || {
            // shoot an image
            Some(camera.shoot())
        },
        |image| {
            // calculate offset
            aligner.align(&image)
        },
        |pos| {
            // slew mount to correct
            mount.slew(pos);
        }
    );
}
