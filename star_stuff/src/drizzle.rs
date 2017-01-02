use std::default::Default;
use std::ops::{AddAssign, DivAssign, Mul};
use image::Image;
use point::{Vector, Point};

pub struct ImageStack<P> {
    image: Image<P>,
    count: usize,
    factor: f32,
    pixel_size: f32,
}

impl<P: Copy + Clone + AddAssign + DivAssign<f32> + Mul<f32, Output=P> + Default> ImageStack<P> {
    pub fn new(width: usize, height: usize, factor: f32) -> Self {
        let w = (width as f32 * factor) as usize;
        let h = (height as f32 * factor) as usize;
        ImageStack {
            image: Image::new(w, h),
            count: 0,
            factor: factor,
            pixel_size: 1.0,
        }
    }

    pub fn add(&mut self, image: &Image<P>, transform: Vector) {
        for y in 0..self.image.height {
            for x in 0..self.image.width {
                let src_pos = Point {x: x as f32, y: y as f32} / self.factor - transform;
                *self.image.pixel_at_mut(x, y) += self.resample(image, src_pos.x, src_pos.y);
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

    fn resample(&self, image: &Image<P>, x: f32, y: f32) -> P {
        // `src` refers to `image`, `dst` refers to `self.image`.
        // `x` and `y` above are in the `src` coordinate system.

        let mut src_val: P = Default::default();
        let dx = x.ceil() - x; // distance to right pixel
        let dy = y.ceil() - y; // distance to bottom pixel
        let dxp = 1.0 - dx; // distance to left pixel
        let dyp = 1.0 - dy; // distance to top pixel
        let dst_w = 1.0 / self.factor; // width & height of dst pixel (in src coords)
        let src_margin = (1.0 - self.pixel_size) / 2.0; // margin around src pixel

        // areas of the four `src` pixels with the `dst` pixel.
        let sw = dx * dyp;
        let nw = dx * dy;
        let ne = dxp * dy;
        let se = dxp * dyp;

        // integer coords of the four `src` pixels
        let e_x = x.ceil() as isize;
        let s_y = y.ceil() as isize;
        let w_x = e_x - 1;
        let n_y = s_y - 1;

        let w = image.width as isize;
        let h = image.height as isize;

        // if the `src` pixel exists, add its weighted value to `src_val`
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
}


#[cfg(test)]
mod tests {
    use test::Bencher;
    use super::*;
    use image::Image;

    fn run_resample_test(pixels: Vec<f32>, x: f32, y: f32, expected: f32) {
        let image = Image {
            width: 3,
            height: 3,
            pixels: pixels,
        };
        let mut stack = ImageStack::new(3, 3, 1.0);
        stack.add(&image, Vector { x: -x, y: -y });
        let v = *stack.into_image().pixel_at(0, 0);
        assert_eq!(v, expected);
    }

    #[test]
    fn test_1() {
        run_resample_test(
            vec![
                0.5, 0.5, 0.5,
                0.5, 1.0, 0.5,
                0.5, 0.5, 0.5,
            ],
            1.0, 1.0,
            1.0
        );
    }

    #[test]
    fn test_2() {
        run_resample_test(
            vec![
                0.5, 0.5, 0.5,
                0.5, 1.0, 0.5,
                0.5, 0.5, 0.5,
            ],
            0.75, 0.75,
            (0.75 * 0.75 * 1.0) + (0.75 * 0.25 * 2.0 * 0.5) + (0.25 * 0.25 * 0.5)
        );
    }

    #[test]
    fn test_edge() {
        run_resample_test(
            vec![
                0.5, 0.5, 0.5,
                0.5, 1.0, 0.5,
                0.5, 0.5, 0.5,
            ],
            -0.75, -0.75,
            0.25 * 0.25 * 0.5
        );
    }

    //#[bench]
    //fn bench_resample(b: &mut Bencher) {
        //let image = Image {
            //width: 3,
            //height: 3,
            //pixels: vec![
                //0.5, 0.5, 0.5,
                //0.5, 1.0, 0.5,
                //0.5, 0.5, 0.5,
            //]
        //};
        //b.iter(|| {
            //resample(&image, 1.0, 1.0);
        //});
    //}

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
        let mut stacker = ImageStack::new(3, 3, 1.0);
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
        let mut stacker = ImageStack::new(3, 3, 1.0);
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
        let mut stacker = ImageStack::new(3, 3, 1.0);
        stacker.add(&image, Vector {x: 0.0, y: 0.0});
        stacker.add(&image, Vector {x: 0.5, y: 0.5});
        assert_eq!(stacker.into_image().pixels, vec![
            0.3125, 0.375, 0.375,
            0.375, 0.8125, 0.5625,
            0.375, 0.5625, 0.5625,
        ]);
    }
}
