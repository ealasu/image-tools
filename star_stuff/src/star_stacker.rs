use std::default::Default;
use std::ops::{AddAssign, DivAssign, Mul};
use image::Image;
use point::{Vector, Point};

pub struct ImageStack<P> {
    image: Image<P>,
    count: usize,
}

impl<P: Copy + Clone + AddAssign + DivAssign<f32> + Mul<f32, Output=P> + Default> ImageStack<P> {
    pub fn new(width: usize, height: usize) -> Self {
        ImageStack {
            image: Image::new(width, height),
            count: 0,
        }
    }

    pub fn add(&mut self, image: &Image<P>, transform: Vector) {
        for y in 0..self.image.height {
            for x in 0..self.image.width {
                let src_pos = Point {x: x as f32, y: y as f32} - transform;
                *self.image.pixel_at_mut(x, y) += resample(image, src_pos.x, src_pos.y);
            }
        }
        self.count += 1;
    }

    pub fn into_image(self) -> Image<P> {
        let count = self.count as f32;
        let mut image = self.image;
        for pixel in image.pixels.iter_mut() {
            *pixel /= count;
        }
        image
    }
}

//pub fn stack(images: &[(String, Vector)], out_path: &str) {
    //// calculate dimensions
    //let d = images.iter().map(|&(ref filename, ref tx)| {
        //let (width, height) = identify(&filename);
        //let top = tx.y as isize;
        //let bottom = height as isize + tx.y as isize;
        //let left = tx.x as isize;
        //let right = width as isize + tx.x as isize;
        //(top, right, bottom, left)
    //}).collect::<Vec<_>>();
    //let right = d.iter().map(|&(_, right, _, _)| right).min().unwrap();
    //let left =  d.iter().map(|&(_, _, _, left)| left).max().unwrap();
    //let width = max(0, right - left) as usize;
    //let bottom = d.iter().map(|&(_, _, bottom, _)| bottom).min().unwrap();
    //let top = d.iter().map(|&(top, _, _, _)| top).max().unwrap();
    //let height = max(0, bottom - top) as usize;
    //let stack_tx = Vector {x: -(left as f32), y: -(top as f32)};
    //assert!(width > 0);
    //assert!(height > 0);

    //// stack
    //let mut stack = ImageStack::new(width, height);
    //for &(ref filename, ref tx) in images.iter() {
        //let image = GrayImage::open(&filename);
        //stack.add(&image, *tx + stack_tx);
    //}
    //let image = stack.into_image();

    //// save
    ////println!("res: {:?}", image);
    ////let res = image::Image::open("data/big-1-c.tiff");
    //image.save(out_path);
//}

fn resample<P: Copy + AddAssign + Mul<f32, Output=P> + Default>(image: &Image<P>, x: f32, y: f32) -> P {
    let mut src_val: P = Default::default();
    let dx = x.ceil() - x;
    let dy = y.ceil() - y;
    let dxp = 1.0 - dx;
    let dyp = 1.0 - dy;

    let sw = dx * dyp;
    let nw = dx * dy;
    let ne = dxp * dy;
    let se = dxp * dyp;

    let e_x = x.ceil() as isize;
    let s_y = y.ceil() as isize;
    let w_x = e_x - 1;
    let n_y = s_y - 1;

    let w = image.width as isize;
    let h = image.height as isize;

    if n_y >= 0 && n_y < h && w_x >= 0 && w_x < w {
        src_val += *image.pixel_at(w_x as usize, n_y as usize) * nw;
    }
    if n_y >= 0 && n_y < h && e_x >= 0 && e_x < w {
        src_val += *image.pixel_at(e_x as usize, n_y as usize) * ne;
    }
    if s_y >= 0 && s_y < h && e_x >= 0 && e_x < w {
        src_val += *image.pixel_at(e_x as usize, s_y as usize) * se;
    }
    if s_y >= 0 && s_y < h && w_x >= 0 && w_x < w {
        src_val += *image.pixel_at(w_x as usize, s_y as usize) * sw;
    }

    src_val
}

#[cfg(test)]
mod tests {
    use test::Bencher;
    use super::*;
    use image::Image;

    #[test]
    fn test_1() {
        let image = Image {
            width: 3,
            height: 3,
            pixels: vec![
                0.5, 0.5, 0.5,
                0.5, 1.0, 0.5,
                0.5, 0.5, 0.5,
            ]
        };
        let v = resample(&image, 1.0, 1.0);
        assert_eq!(v, 1.0);
    }

    #[test]
    fn test_2() {
        let image = Image {
            width: 3,
            height: 3,
            pixels: vec![
                0.5, 0.5, 0.5,
                0.5, 1.0, 0.5,
                0.5, 0.5, 0.5,
            ]
        };
        let v = resample(&image, 0.75, 0.75);
        assert_eq!(v, (0.75 * 0.75 * 1.0) + (0.75 * 0.25 * 2.0 * 0.5) + (0.25 * 0.25 * 0.5));
    }

    #[test]
    fn test_edge() {
        let image = Image {
            width: 3,
            height: 3,
            pixels: vec![
                0.5, 0.5, 0.5,
                0.5, 1.0, 0.5,
                0.5, 0.5, 0.5,
            ]
        };
        let v = resample(&image, -0.75, -0.75);
        assert_eq!(v, 0.25 * 0.25 * 0.5);
    }

    #[bench]
    fn bench_resample(b: &mut Bencher) {
        let image = Image {
            width: 3,
            height: 3,
            pixels: vec![
                0.5, 0.5, 0.5,
                0.5, 1.0, 0.5,
                0.5, 0.5, 0.5,
            ]
        };
        b.iter(|| {
            resample(&image, 1.0, 1.0);
        });
    }

    #[test]
    fn test_stack_1() {
        let image: Image<f32> = Image {
            width: 3,
            height: 3,
            pixels: vec![
                0.5, 0.5, 0.5,
                0.5, 1.0, 0.5,
                0.5, 0.5, 0.5,
            ]
        };
        let mut stacker = ImageStack::new(3, 3);
        stacker.add(&image, Vector {x: 0.0, y: 0.0});
        assert_eq!(stacker.into_image().pixels, vec![
            0.5, 0.5, 0.5,
            0.5, 1.0, 0.5,
            0.5, 0.5, 0.5,
        ]);
    }

    #[test]
    fn test_stack_2() {
        let image = Image {
            width: 3,
            height: 3,
            pixels: vec![
                0.5, 0.5, 0.5,
                0.5, 1.0, 0.5,
                0.5, 0.5, 0.5,
            ]
        };
        let mut stacker = ImageStack::new(3, 3);
        stacker.add(&image, Vector {x: 0.0, y: 0.0});
        stacker.add(&image, Vector {x: 0.0, y: 0.0});
        assert_eq!(stacker.into_image().pixels, vec![
            0.5, 0.5, 0.5,
            0.5, 1.0, 0.5,
            0.5, 0.5, 0.5,
        ]);
    }

    #[test]
    fn test_stack_3() {
        let image = Image {
            width: 3,
            height: 3,
            pixels: vec![
                0.5, 0.5, 0.5,
                0.5, 1.0, 0.5,
                0.5, 0.5, 0.5,
            ]
        };
        let mut stacker = ImageStack::new(3, 3);
        stacker.add(&image, Vector {x: 0.0, y: 0.0});
        stacker.add(&image, Vector {x: 0.5, y: 0.5});
        assert_eq!(stacker.into_image().pixels, vec![
            0.3125, 0.375, 0.375,
            0.375, 0.8125, 0.5625,
            0.375, 0.5625, 0.5625,
        ]);
    }
}
