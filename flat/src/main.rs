extern crate docopt;
extern crate rustc_serialize;
extern crate image;
extern crate star_stuff;
extern crate rayon;

use docopt::Docopt;
use image::Image;
use rayon::prelude::*;


const USAGE: &'static str = "
Usage:
    flat <output> <input>...
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_output: String,
    arg_input: Vec<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let first = Image::<u16>::open_raw(&args.arg_input[0]);
    let (w, h) = (first.width, first.height);
    let count = args.arg_input.len() as f32;
    let img = args.arg_input
        .into_par_iter()
        .map(|f| {
            println!("stacking {}", f);
            Image::<u16>::open_raw(&f).to_f32()
        })
        .reduce(|| Image::<f32>::new(w, h), |a, b| a + b);
    let img = img / count;

    println!("min: {}", img.min());
    println!("max: {}", img.max());
    println!("avg: {}", img.average());

    let mut r_max = 0.0;
    let mut r_count = 0;
    let mut g_max = 0.0;
    let mut g_count = 0;
    let mut b_max = 0.0;
    let mut b_count = 0;
    let img = img.to_rggb();
    for p in img.pixels.iter() {
        if p.rc > 0.0 && p.r > r_max {
            r_max += p.r;
            r_count += 1;
        }
        if p.gc > 0.0 && p.g > g_max {
            g_max += p.g;
            g_count += 1;
        }
        if p.bc > 0.0 && p.b > b_max {
            b_max += p.b;
            b_count += 1;
        }
    }
    r_max = r_max / r_count as f32;
    g_max = g_max / g_count as f32;
    b_max = b_max / b_count as f32;
    println!("r_max: {}", r_max);
    println!("g_max: {}", g_max);
    println!("b_max: {}", b_max);

    let img = img.map(|p| {
        if p.rc > 0.0 {
            p.r / r_max
        } else if p.gc > 0.0 {
            p.g / g_max
        } else if p.bc > 0.0 {
            p.b / b_max
        } else {
            unreachable!()
        }
    });

    println!("min: {}", img.min());
    println!("max: {}", img.max());
    println!("avg: {}", img.average());

    img.save_fits(&args.arg_output);
    //img.center_crop(400, 400).save_fits(&args.arg_output);
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
        println!("before min: {}, max: {}", img.min(), img.max());
        img.save("before.tif");
        img /= flat;
        println!("after min: {}, max: {}", img.min(), img.max());
        img.save("after.tif");
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
