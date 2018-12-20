#![feature(fs_read_write)]

extern crate image;
extern crate rawspeed;
extern crate turbojpeg;
extern crate glob;
extern crate ndarray;
extern crate crossbeam_utils;
extern crate crossbeam_channel;

use std::fs;
use glob::glob;
use crossbeam_utils::scoped::scope;
use crossbeam_channel::{bounded, unbounded};
use ndarray::prelude::*;
use image::prelude::*;

pub fn write_jpeg(img: &Array2<f64>, count: usize) {
    let mut img = img.clone();
    img /= count as f64;
    println!("minmax: {:?}", img.min_max());
    let img = img.mapv(|v| {
        v.powf(1.0 / 3.2)
    });
    println!("minmax after gamma: {:?}", img.min_max());
    //let img = img.stretch::<u8>(0.0, 0.04, 0, 255).to_rgb();
    let img = img.stretch_to_bounds::<u8>().to_rgb();
    fs::write(format!("out/stacked_{:04}.jpg", count), turbojpeg::compress(&img).unwrap()).unwrap();
}

pub fn stack() {
    let (in_tx, in_rx) = unbounded();
    let (out_tx, out_rx) = bounded(1);
    //let pattern = "/Volumes/data/photos/2017/2017-12-16-darks/101CANON/*.CR2";
    //let pattern = "/Volumes/data/photos/2017/2017-12-16-bias-frames/*.CR2";
    //let pattern = "test/*.CR2";
    let pattern = "test/IMG_9445.CR2";
    for entry in glob(pattern).unwrap() {
        in_tx.send(entry.unwrap()).unwrap();
    }
    drop(in_tx);
    let img = scope(|scope| {
        for _ in 0..3 {
            let out_tx = out_tx.clone();
            let in_rx = in_rx.clone();
            scope.spawn(move || {
                for filename in in_rx.iter() {
                    let img = rawspeed::decode(&fs::read(filename).unwrap()).unwrap()
                        //.center_crop(600, 600)
                        .scale_to_f64();
                    out_tx.send(img).unwrap();
                }
            });
        }
        drop(out_tx);
        let mut stack = out_rx.recv().unwrap();
        let mut count: usize = 1;
        write_jpeg(&stack, count);
        for img in out_rx.iter() {
            stack += &img;
            count += 1;
            println!("{}", count);
            write_jpeg(&stack, count);
        }
        stack /= count as f64;
        stack
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        stack();
        // 1. read a bias frame, stretch & save as jpeg
        //stretch_raw("data/bias-frames/IMG_1010.CR2", "out/bias.jpg");
        //stretch_raw("test/bias-frame.CR2", "out/bias-frame.jpg", false);
        //stretch_raw("test/dark-frame.CR2", "out/dark-frame.jpg", false);
        //stretch_raw("test/dark-frame.CR2", "out/dark-frame-minus-bias.jpg", true);
        //stretch_raw("test/dark-frame-2.CR2", "out/dark-frame-2.jpg", false);
        //

        //let mut stack = None;
        //let mut count: usize = 0;
        //for entry in glob("/Volumes/data/photos/2017/2017-12-16-darks/101CANON/*.CR2").unwrap() {
            //let entry = entry.unwrap();
            //println!("{}", entry.display());
            //let img = rawspeed::decode(&fs::read(entry).unwrap()).unwrap().scale_to_f32();
            //stack = Some(if let Some(mut stack) = stack {
                //stack += &img;
                //count += 1;
                //stack
            //} else {
                //img
            //})
        //}
        //let mut img = stack.unwrap();
        //println!("minmax: {:?}", img.min_max());
    }

    //fn stretch_raw(input: &str, output: &str, sub_bias: bool) {
        //let bias = rawspeed::decode(&fs::read("test/bias-frame.CR2").unwrap()).unwrap()
            ////.center_crop(400, 400)
            //.scale_to_f32();
        //let mut img = rawspeed::decode(&fs::read(input).unwrap()).unwrap()
            ////.center_crop(400, 400)
            //.scale_to_f32();
        //if sub_bias {
            //img -= &bias;
        //}
        ////let img = img.stretch_to_bounds::<u8>();
        //let img = img.stretch::<u8>(0.0, 0.1, 0, 255);
        //println!("minmax: {:?}", img.min_max());
        ////println!("{:?}", &img.pixels()[991..999]);
        ////let img = img.invert();
        ////println!("{:?}", &img.pixels()[991..999]);
        //let img = img.to_rgb();
        //fs::write(format!("{}.dat", output), img.as_bytes()).unwrap();
        //fs::write(output, turbojpeg::compress(&img).unwrap()).unwrap();
    //}
}
