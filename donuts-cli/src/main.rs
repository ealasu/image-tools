extern crate donuts;
extern crate image;

use std::env;
use image::Image;

fn main() {
    let mut args = env::args();
    args.next();
    let ref_path = args.next().unwrap();
    let sample_path = args.next().unwrap();

    let ref_img = Image::<f32>::open_jpeg_file(ref_path).center_crop(900, 900);
    let sample_img = Image::<f32>::open_jpeg_file(sample_path).center_crop(900, 900);

    let ref_p = donuts::preprocess_image(ref_img);
    let sample_p = donuts::preprocess_image(sample_img);

    let (x, y) = donuts::align(&ref_p, &sample_p);
    println!("{},{}", x, y);
}
