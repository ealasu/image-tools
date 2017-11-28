extern crate image;
extern crate star_stuff;
extern crate rayon;
extern crate structopt;
#[macro_use] extern crate structopt_derive;

use structopt::StructOpt;
use image::Image;
use rayon::prelude::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "flat", about = "")]
struct Args {
    #[structopt(long = "output")]
    flag_output: String,
    #[structopt(long = "input")]
    arg_input: Vec<String>,
}

fn main() {
    let args = Args::from_args();
    let first = Image::<u16>::open_raw(&args.arg_input[0]);
    let (w, h) = (first.width, first.height);
    let count = args.arg_input.len() as f64;
    let img = args.arg_input
        .into_par_iter()
        .map(|f| {
            println!("stacking {}", f);
            Image::<u16>::open_raw(&f).into_f64()
        })
        .reduce(|| Image::<f64>::new(w, h), |a, b| a + b);
    let img = img / count;

    println!("min: {}", img.min());
    println!("max: {}", img.max());
    println!("avg: {}", img.average());

    let img = img.to_rggb();
    let (r_avg, g_avg, b_avg) = img.avg();
    println!("r_avg: {}", r_avg);
    println!("g_avg: {}", g_avg);
    println!("b_avg: {}", b_avg);

    let img = img.map(|p| {
        if p.rc > 0.0 {
            p.r / r_avg
        } else if p.gc > 0.0 {
            p.g / g_avg
        } else if p.bc > 0.0 {
            p.b / b_avg
        } else {
            unreachable!()
        }
    });

    println!("min: {}", img.min());
    println!("max: {}", img.max());
    println!("avg: {}", img.average());

    img.save_fits(&args.flag_output);
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::*;

    #[test]
    fn test_1() {
        let flat = Image::<f32>::open_fits("flat.fits");
        println!("flat min: {}, max: {}", flat.min(), flat.max());
        println!("first pixels: {:?}", &flat.pixels[..5]);
        println!("last pixels: {:?}", &flat.pixels[flat.pixels.len() - 5..]);

        let mut img = Image::<u16>::open_raw("test.cr2").to_f32();

        img.save("before-nc.jpg");
        let img_b = img.to_rggb();
        println!("before avg: {:?}", img_b.avg());
        let im2 = img_b.correct_white_balance();
        println!("before c avg: {:?}", im2.avg());
        //println!("before min: {}, max: {}", im2.min(), im2.max());
        im2.to_rgb().to_gray().save("before.jpg");

        img /= flat;

        img.save("after-nc.jpg");
        let img_b = img.to_rggb();
        println!("after avg: {:?}", img_b.avg());
        let im2 = img_b.correct_white_balance();
        println!("after c avg: {:?}", im2.avg());
        im2.to_rgb().to_gray().save("after.jpg");
        //println!("after min: {}, max: {}", im2.min(), im2.max());

        //println!("max: {}", img.max());
        //println!("range: {}", img.max() - img.min());
        //println!("avg: {}", img.average());
    }

//    #[test]
//    fn test_1() {
//        let mut chan: Channel<f32> = Channel::from_data(8, 4, vec![
//            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 
//            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 
//            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 
//            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 
//        ]);
//        find_delta(&mut chan);
//        assert_eq!(*chan.pixels(), vec![
//            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 
//            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 
//            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 
//            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 
//        ]);
//    }
//
//    #[test]
//    fn test_2() {
//        let mut chan: Channel<f32> = Channel::from_data(8, 4, vec![
//            3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 
//            1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 
//            3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 
//            1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 
//        ]);
//        find_delta(&mut chan);
//        assert_eq!(*chan.pixels(), vec![
//            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 
//            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 
//            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 
//            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 
//        ]);
//    }
//
//    #[test]
//    fn test_3() {
//        let mut chan: Channel<f32> = Channel::from_data(8, 4, vec![
//            3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 
//            1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 
//            3.0, 1.0, 1.0, 1.0, 3.0, 1.0, 3.0, 1.0, 
//            1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 
//        ]);
//        find_delta(&mut chan);
//        assert_eq!(*chan.pixels(), vec![
//            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 
//            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 
//            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 
//            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 
//        ]);
//    }
}
