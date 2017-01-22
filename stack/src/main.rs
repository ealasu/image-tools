extern crate docopt;
extern crate rustc_serialize;
extern crate star_stuff;
extern crate donuts;
extern crate image;
extern crate align_api;
extern crate crossbeam;

use std::thread;
use std::sync::Mutex;
use std::sync::mpsc::sync_channel;
use docopt::Docopt;
use star_stuff::drizzle::{self, ImageStack};
use image::{Image, Rgb, RgbBayer, ImageKind};
use crossbeam::sync::chase_lev;


const USAGE: &'static str = "
Stacker.

Usage:
    stack --output=<filename> --flat=<filename> --alignment=<filename>
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_output: String,
    flag_flat: String,
    flag_alignment: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    stack(args);
    //align(args);
}

fn align(args: Args) {
    let factor: f64 = 1.0;
    let flat = if let ImageKind::F64(v) = ImageKind::open_fits(&args.flag_flat) {
        v
    } else {
        panic!();
    };
    let alignment = align_api::read(&args.flag_alignment);
    let mut first_img = Image::<u16>::open_raw(&alignment[0].filename).to_f32().to_f64();
    first_img /= &flat;
    let first_img = first_img.to_rggb();

    for (i, file) in alignment.iter().enumerate() {
        println!("adding {}", i);
        let mut img = Image::<u16>::open_raw(&file.filename).to_f32().to_f64();
        let transform = file.transform.to_f64();
        img /= &flat;
        let img = img.to_rggb();
        let mut stack = first_img.clone();
        drizzle::add(&mut stack, &img, transform, factor, 0.80);
        stack
            .to_rgb()
            .remove_background(1.0)
            .crop(img.width - 1500, img.height / 2 - 1000/2, 1000, 1000)
            .gamma(1.0 / 2.2)
            .stretch(0.0, 1.0)
            .to_f32()
            .save(&format!("{}/{}.jpg", args.flag_output, i));
    }

    //for y in 0..img.height {
        //*img.pixel_at_mut(900 / 2, y) *= 0.5;
    //}
    //for x in 0..img.width {
        //*img.pixel_at_mut(x, 900 / 2) *= 0.5;
    //}
}

fn stack(args: Args) {
    println!("processing ref image");
    let factor: f64 = 1.0;
    let flat = if let ImageKind::F64(v) = ImageKind::open_fits(&args.flag_flat) {
        v
    } else {
        panic!();
    };
    //raw_ref
        //.to_f32()
        //.center_crop(900, 900)
        //.save("ref-bayer.tif");
    //raw_ref
        //.to_rggb()
        //.to_rgb()
        //.center_crop(900, 900)
        //.save("ref-bayer-rgb.tif");
    //raw_ref
        //.to_rggb()
        //.to_green_interpolated()
        //.center_crop(900, 900)
        //.save("ref-bayer-green-inter.tif");
    //let ref_image = raw_ref
        //.to_rggb()
        //.to_green()
        //.center_crop(900, 900);
    //ref_image.save("ref.tif");
    //let ref_image = raw_ref
        //.to_rggb()
        //.to_green_interpolated()
        //.center_crop(900, 900);

    let alignment = align_api::read(&args.flag_alignment);

    let (tx, rx) = sync_channel(0);
    let (mut worker, stealer) = chase_lev::deque();
    for file in alignment.iter() {
        worker.push(file);
    }

    crossbeam::scope(|scope| {
        for _ in 0..2 {
            let flat = flat.clone();
            let tx = tx.clone();
            let stealer = stealer.clone();
            scope.spawn(move || {
                loop {
                    let file = match stealer.steal() {
                        chase_lev::Steal::Data(d) => d,
                        chase_lev::Steal::Abort => continue,
                        chase_lev::Steal::Empty => break
                    };
                    let mut img = Image::<u16>::open_raw(&file.filename);

                    let transform = file.transform.to_f64();
                    tx.send((img, transform)).unwrap();
                }
                println!("thread done.");
            });
        }
        drop(tx);

        let img = rx.iter().enumerate().fold(None, |stack, (i, (img, transform))| {
            println!("adding {}", i);
            let mut img = img.to_f32().to_f64();
            img /= &flat;
            let img = img.to_rggb();
            if let Some(mut stack) = stack {
                drizzle::add(&mut stack, &img, transform, factor, 0.80);
                Some(stack)
            } else {
                Some(img)
            }
        }).unwrap();

        img.to_rgb().save_fits(&args.flag_output);
        let holes = img.center_crop(900, 900).holes();
        println!("holes min/max: {:?}", holes.min_max());
        holes.to_u8().save_jpeg_file("holes.jpg");
    });
}
