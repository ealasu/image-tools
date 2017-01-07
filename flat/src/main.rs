extern crate docopt;
extern crate rustc_serialize;
extern crate image;
extern crate star_stuff;

use docopt::Docopt;
use star_stuff::stack::ImageStack;
use image::*;


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
    let mut stack = ImageStack::new(first.width, first.height);
    for f in args.arg_input.iter() {
        println!("stacking {}", f);
        let img = Image::<u16>::open_raw(f).to_f32();
        stack.add(&img);
    }
    let img = stack.finish();

    println!("min: {}", img.min());
    println!("max: {}", img.max());
    println!("avg: {}", img.average());

    let mut r_max = 0.0;
    let mut g_max = 0.0;
    let mut b_max = 0.0;
    let img = img.to_rggb();
    for p in img.pixels.iter() {
        if p.rc > 0.0 && p.r > r_max {
            r_max = p.r;
        }
        if p.gc > 0.0 && p.g > g_max {
            g_max = p.g;
        }
        if p.bc > 0.0 && p.b > b_max {
            b_max = p.b;
        }
    }
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
    img.save(&args.arg_output);
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//    use image::*;
//
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
//}
