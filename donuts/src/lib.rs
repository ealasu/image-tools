#![feature(test)]
#![feature(iter_max_by)]

#[cfg(test)] extern crate test;
#[cfg(test)] extern crate rand;
extern crate statistical;
extern crate quickersort;
extern crate image;

mod remove_background;
mod projection;
mod correlation;

use image::GrayImage;

pub struct Projection {
    x: Vec<f32>,
    y: Vec<f32>,
}

pub fn preprocess_image(mut image: GrayImage<f32>) -> Projection {
    remove_background::remove_background(&mut image, 32);
    Projection {
        x: projection::x_projection(&image),
        y: projection::y_projection(&image),
    }
}

pub fn align(reference: &Projection, sample: &Projection) -> (f32, f32) {
    let n = 100;
    let x = correlation::calc_offset(&reference.x[..], &sample.x[..], n);
    let y = correlation::calc_offset(&reference.y[..], &sample.y[..], n);
    (x, y)
}
