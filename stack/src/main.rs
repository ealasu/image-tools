extern crate star_stuff;
extern crate image;
extern crate align_api;
extern crate crossbeam;
extern crate geom;
extern crate structopt;
#[macro_use] extern crate structopt_derive;

use std::sync::mpsc::sync_channel;
use image::{Image, Rgb, ImageKind};
use crossbeam::sync::chase_lev;
use structopt::StructOpt;
use stack_methods::StackMethod;

#[derive(StructOpt, Debug)]
#[structopt(name = "stack", about = "")]
struct Opt {
    #[structopt(long = "alignment", help = "Alignment json file")]
    alignment: String,
    #[structopt(long = "flat", help = "FITS file of flat field")]
    flat: String,
    #[structopt(long = "output", help = "Filename of output FITS file")]
    output: String,
    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(StructOpt, Debug)]
enum Cmd {
    #[structopt(name = "average", about = "Averages images")]
    Average {
        #[structopt(long = "pixel-aperture")]
        pixel_aperture: f64,
    },
    #[structopt(name = "sigma-kappa")]
    SigmaKappa {
        #[structopt(long = "pixel-aperture")]
        pixel_aperture: f64,
        #[structopt(long = "average", help = "FITS file of average")]
        average: String,
        #[structopt(long = "kappa")]
        kappa: f64,
    }
}

fn main() {
    let opt = Opt::from_args();
    //println!("{:?}", opt);
    match opt.cmd {
        Cmd::Average { pixel_aperture } => {
            stack(
                &opt.alignment,
                &opt.flat,
                stack_methods::Average { pixel_aperture },
                &opt.output);
        }
        Cmd::SigmaKappa { pixel_aperture, average, kappa } => {
            stack(
                &opt.alignment,
                &opt.flat,
                stack_methods::SigmaKappa {
                    pixel_aperture,
                    average: open_fits_rgb(&average),
                    kappa
                },
                &opt.output);
        }
    }
}

fn stack<S>(alignment: &str, flat: &str, stack_method: S, output: &str)
where S: StackMethod {
    let flat = open_fits_gray(flat);
    let alignment = align_api::read(alignment);
    let img = for_each_image(
        alignment,
        || |file| {
            (Image::<u16>::open_raw(&file.filename), file.transform.to_f64())
        },
        |stack, (img, transform)| {
            let mut img = img.to_f32().to_f64();
            img /= &flat;
            stack_method.stack(stack, img, transform)
        }
    ).unwrap();
    img.to_rgb().save_fits(output);

    //let holes = img.center_crop(900, 900).holes();
    //println!("holes min/max: {:?}", holes.min_max());
    //holes.to_u8().save_jpeg_file("holes.jpg");
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
    let (worker, stealer) = chase_lev::deque();
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

fn open_fits_gray(filename: &str) -> Image<f64> {
    if let ImageKind::F64(v) = ImageKind::open_fits(filename) {
        v
    } else {
        panic!("wrong fits type")
    }
}

fn open_fits_rgb(filename: &str) -> Image<Rgb<f64>> {
    if let ImageKind::RgbF64(v) = ImageKind::open_fits(filename) {
        v
    } else {
        panic!("wrong fits type")
    }
}

pub mod stack_methods {
    use image::{Image, Rgb, RgbBayer};
    use star_stuff::drizzle;
    use geom::Matrix3x3;

    pub trait StackMethod {
        fn stack(&self, stack: Option<Image<RgbBayer<f64>>>, img: Image<f64>, transform: Matrix3x3<f64>) -> Option<Image<RgbBayer<f64>>>;
    }

    pub struct Average {
        pub pixel_aperture: f64,
    }

    impl StackMethod for Average {
        fn stack(&self, stack: Option<Image<RgbBayer<f64>>>, img: Image<f64>, transform: Matrix3x3<f64>) -> Option<Image<RgbBayer<f64>>> {
             let img = img.to_rggb();
             if let Some(mut stack) = stack {
                 drizzle::add(&mut stack, &img, transform, 1.0, self.pixel_aperture, |_,_,_| true);
                 Some(stack)
             } else {
                 Some(img)
             }
        }
    }

    pub struct SigmaKappa {
        pub pixel_aperture: f64,
        pub kappa: f64,
        pub average: Image<Rgb<f64>>,
    }

    impl StackMethod for SigmaKappa {
        fn stack(&self, stack: Option<Image<RgbBayer<f64>>>, img: Image<f64>, transform: Matrix3x3<f64>) -> Option<Image<RgbBayer<f64>>> {
             let img = img.to_rggb();
             if let Some(mut stack) = stack {
                 drizzle::add(&mut stack, &img, transform, 1.0, self.pixel_aperture, |x, y, p| {
                     let avg = self.average.pixel_at(x, y);
                     (p.rc < 0.2 || (p.r / p.rc - avg.r).abs() < self.kappa) &&
                     (p.gc < 0.2 || (p.g / p.gc - avg.g).abs() < self.kappa) &&
                     (p.bc < 0.2 || (p.b / p.bc - avg.b).abs() < self.kappa)
                 });
                 Some(stack)
             } else {
                 Some(img)
             }
        }
    }
}
