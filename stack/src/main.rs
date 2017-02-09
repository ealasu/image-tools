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
    stack average --output=<filename> --flat=<filename> --alignment=<filename>
    stack median --output=<filename> --flat=<filename> --alignment=<filename> --average=<filename>
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_average: bool,
    cmd_median: bool,
    flag_output: String,
    flag_flat: String,
    flag_alignment: String,
    flag_average: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    if args.cmd_average {
        stack_average(args);
    } else if args.cmd_median {
        stack_median(args);
    }
}

//fn align(args: Args) {
    //let factor: f64 = 1.0;
    //let flat = if let ImageKind::F64(v) = ImageKind::open_fits(&args.flag_flat) {
        //v
    //} else {
        //panic!();
    //};
    //let alignment = align_api::read(&args.flag_alignment);
    //let mut first_img = Image::<u16>::open_raw(&alignment[0].filename).to_f32().to_f64();
    //first_img /= &flat;
    //let first_img = first_img.to_rggb();

    //let total = alignment.len();
    //for (i, file) in alignment.iter().enumerate() {
        //println!("adding {} of {}", i, total);
        //let mut img = Image::<u16>::open_raw(&file.filename).to_f32().to_f64();
        //let transform = file.transform.to_f64();
        //img /= &flat;
        //let img = img.to_rggb();
        //let mut stack = first_img.clone();
        //drizzle::add(&mut stack, &img, transform, factor, 0.80);
        //stack
            //.to_rgb()
            //.remove_background(1.0)
            //.crop(img.width - 1500, img.height / 2 - 1000/2, 1000, 1000)
            //.gamma(1.0 / 2.2)
            //.stretch(0.0, 1.0)
            //.to_f32()
            //.save(&format!("{}/{}.jpg", args.flag_output, i));
    //}

    ////for y in 0..img.height {
        ////*img.pixel_at_mut(900 / 2, y) *= 0.5;
    ////}
    ////for x in 0..img.width {
        ////*img.pixel_at_mut(x, 900 / 2) *= 0.5;
    ////}
//}

fn stack_average(args: Args) {
    let flat = open_fits(&args.flag_flat);
    let alignment = align_api::read(&args.flag_alignment);
    let img = for_each_image(
        alignment,
        || |file| {
             (Image::<u16>::open_raw(&file.filename), file.transform.to_f64())
        },
        |stack, (img, transform)| {
             let mut img = img.to_f32().to_f64();
             img /= &flat;
             let img = img.to_rggb();
             if let Some(mut stack) = stack {
                 drizzle::add(&mut stack, &img, transform, 1.0, 0.80);
                 Some(stack)
             } else {
                 Some(img)
             }
        }
    ).unwrap();

    img.to_rgb().save_fits(&args.flag_output);
    //let holes = img.center_crop(900, 900).holes();
    //println!("holes min/max: {:?}", holes.min_max());
    //holes.to_u8().save_jpeg_file("holes.jpg");
}

fn stack_median(args: Args) {
    let flat = open_fits(&args.flag_flat);
    let average = open_fits(&args.flag_average);
    let alignment = align_api::read(&args.flag_alignment);
    let img = for_each_image(
        alignment,
        || |file| {
             (Image::<u16>::open_raw(&file.filename), file.transform.to_f64())
        },
        |stack, (img, transform)| {
             let mut img = img.to_f32().to_f64();
             img /= &flat;
             let img = img.to_rggb();
             if let Some(mut stack) = stack {
                 // TODO
                 drizzle::add(&mut stack, &img, transform, 1.0, 0.80);
                 Some(stack)
             } else {
                 Some(img)
             }
        }
    ).unwrap();

    img.to_rgb().save_fits(&args.flag_output);
}

fn for_each_image<Item,MapFnFactory,MapFn,MappedItem,ReduceFn, ReducedItem>(
    items: Vec<Item>, map: MapFnFactory, reduce: ReduceFn) -> Option<ReducedItem>
where
    Item: Send,
    MappedItem: Send,
    MapFnFactory: Fn() -> MapFn,
    MapFn: Fn(Item) -> MappedItem, MapFn: Send,
    ReduceFn: Fn(Option<ReducedItem>, MappedItem) -> Option<ReducedItem>
{
    let (tx, rx) = sync_channel(0);
    let (mut worker, stealer) = chase_lev::deque();
    let total = items.len();
    for item in items.into_iter() {
        worker.push(item);
    }

    crossbeam::scope(|scope| {
        for _ in 0..2 {
            let tx = tx.clone();
            let stealer = stealer.clone();
            let map = map();
            scope.spawn(move || {
                loop {
                    let item = match stealer.steal() {
                        chase_lev::Steal::Data(d) => d,
                        chase_lev::Steal::Abort => continue,
                        chase_lev::Steal::Empty => break
                    };
                    let mapped_item = map(item);
                    tx.send(mapped_item).unwrap();
                }
                println!("thread done.");
            });
        }
        drop(tx);

        rx.iter().enumerate().fold(None, |acc, (i, mapped_item)| {
            println!("processing image {} of {}", i, total);
            reduce(acc, mapped_item)
        })
    })
}

fn open_fits(filename: &str) -> Image<f64> {
    if let ImageKind::F64(v) = ImageKind::open_fits(filename) {
        v
    } else {
        panic!("wrong fits type")
    }
}
