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
    stack --output=<filename> --flat=<filename> --alignment=<filename> <input>...
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_output: String,
    flag_flat: String,
    flag_alignment: String,
    arg_input: Vec<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    stack(args);
}

//fn align(args: Args) {
//        stack.add(&sample_image, Vector { x:x, y:y });
//        let mut img = stack.finish();
//        for y in 0..img.height {
//            *img.pixel_at_mut(900 / 2, y) *= 0.5;
//        }
//        for x in 0..img.width {
//            *img.pixel_at_mut(x, 900 / 2) *= 0.5;
//        }
//        img.save(&format!("{}/{}.jpg", args.flag_output, i));
//}

fn stack(args: Args) {
    println!("processing ref image");
    let factor: f64 = 1.0;
    let raw_ref = Image::<u16>::open_raw(&args.arg_input[0]);
    let w = (raw_ref.width as f64 * factor) as usize;
    let h = (raw_ref.height as f64 * factor) as usize;
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
    for file in args.arg_input.iter() {
        worker.push(file);
    }

    crossbeam::scope(|scope| {
        for _ in 0..1 {
            let input = args.arg_input.clone();
            let alignment = alignment.clone();
            let flat = flat.clone();
            let tx = tx.clone();
            scope.spawn(move || {
                for file in input.iter() {
                    let mut img = Image::<u16>::open_raw(&file);
                    let mut img = img.to_f32().to_f64();
                    img /= &flat;

                    let transform = alignment
                        .iter()
                        .find(|i| &i.filename == file)
                        .ok_or_else(|| format!("missing alignment for {}", file))
                        .unwrap()
                        .transform.to_f64();

                    tx.send((img, transform)).unwrap();
                }
                println!("thread done.");
            });
        }
        drop(tx);

        let mut stack = Image::<RgbBayer<f64>>::new(w, h);
        for (img, transform) in rx.iter() {
            let img = img.to_rggb();
            println!("adding");
            drizzle::add(&mut stack, &img, transform, factor, 0.80);
        }

        let img = stack;
        img.to_rgb().save_fits(&args.flag_output);
        let holes = img.center_crop(900, 900).holes();
        println!("holes min/max: {:?}", holes.min_max());
        holes.to_u8().save_jpeg_file("holes.jpg");
    });
}
