use std::default::Default;
use std::ops::{AddAssign, DivAssign, Mul};
use image::Image;
use point::{Vector, Point};

#[inline(always)]
fn min(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}

#[inline(always)]
fn max(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

#[inline(always)]
fn positive(v: f32) -> f32 {
    if v > 0.0 { v } else { 0.0 }
}

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
        // `x` and `y` above are the origin of the `dst` pixel in the `src` coordinate system.

        let mut src_val: P = Default::default();

        // TODO
        //let src_margin = (1.0 - self.pixel_size) / 2.0; // margin around src pixel

        let dst_size = 1.0 / self.factor; // width & height of dst pixel (in src coords)

        // east, or right
        let e = positive((x + dst_size) - x.ceil());
        // south, or bottom
        let s = positive((y + dst_size) - y.ceil());
        // west, or left
        let w = min(x + dst_size, x.ceil()) - x;
        // north, or top
        let n = min(y + dst_size, y.ceil()) - y;

        // areas of intersection of the four `src` pixels with the `dst` pixel.
        let nw = n * w;
        let se = s * e;
        let sw = s * w;
        let ne = n * e;

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
        run_resample_test_with_factor(1.0, pixels, x, y, expected);
    }

    fn run_resample_test_with_factor(factor: f32, pixels: Vec<f32>, x: f32, y: f32, expected: f32) {
        let image = Image {
            width: 3,
            height: 3,
            pixels: pixels,
        };
        let mut stack = ImageStack::new(3, 3, factor);
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


    #[test]
    fn test_factor() {
        let run = |x, y, expected| {
            run_resample_test_with_factor(
                2.0,
                vec![
                    0.5, 0.5, 0.5,
                    0.5, 1.0, 0.5,
                    0.5, 0.5, 0.5,
                ],
                x, y,
                expected
            );
        };
        run(-3.0, -3.0, 0.0);
        run(0.0, 0.0, 0.125);
        run(0.5, 0.5, 0.125);
        run(1.0, 1.0, 0.25);
        run(1.5, 1.5, 0.25);
        run(2.0, 2.0, 0.125);
        run(2.5, 2.5, 0.125);
        run(3.0, 3.0, 0.0);

        run(1.75, 1.0, 0.25 / 2.0 + 0.125 / 2.0);
        run(1.0, 1.75, 0.25 / 2.0 + 0.125 / 2.0);
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

    fn run_stack_test(pixels: Vec<f32>, x: f32, y: f32, expected: Vec<f32>) {
        let image: Image<f32> = Image {
            width: 3,
            height: 3,
            pixels: pixels,
        };
        let mut stacker = ImageStack::new(3, 3, 1.0);
        stacker.add(&image, Vector {x: x, y: y});
        assert_eq!(stacker.into_image().pixels, expected);
    }

    fn run_stack_test_2(pixels1: Vec<f32>, x1: f32, y1: f32, pixels2: Vec<f32>, x2: f32, y2: f32, expected: Vec<f32>) {
        let mut stacker = ImageStack::new(3, 3, 1.0);
        let image1: Image<f32> = Image {
            width: 3,
            height: 3,
            pixels: pixels1,
        };
        stacker.add(&image1, Vector {x: x1, y: y1});
        let image2: Image<f32> = Image {
            width: 3,
            height: 3,
            pixels: pixels2,
        };
        stacker.add(&image2, Vector {x: x2, y: y2});
        assert_eq!(stacker.into_image().pixels, expected);
    }

    #[test]
    fn test_stack_1() {
        run_stack_test(
            vec![
                0.5, 0.5, 0.5,
                0.5, 1.0, 0.5,
                0.5, 0.5, 0.5,
            ],
            0.0, 0.0,
            vec![
                0.5, 0.5, 0.5,
                0.5, 1.0, 0.5,
                0.5, 0.5, 0.5,
            ]
        );
    }

    #[test]
    fn test_stack_2() {
        run_stack_test_2(
            vec![
                0.5, 0.5, 0.5,
                0.5, 1.0, 0.5,
                0.5, 0.5, 0.5,
            ],
            0.0, 0.0,
            vec![
                0.5, 0.5, 0.5,
                0.5, 1.0, 0.5,
                0.5, 0.5, 0.5,
            ],
            0.0, 0.0,
            vec![
                0.5, 0.5, 0.5,
                0.5, 1.0, 0.5,
                0.5, 0.5, 0.5,
            ]
        );
    }

    #[test]
    fn test_stack_3() {
        run_stack_test_2(
            vec![
                0.5, 0.5, 0.5,
                0.5, 1.0, 0.5,
                0.5, 0.5, 0.5,
            ],
            0.0, 0.0,
            vec![
                0.5, 0.5, 0.5,
                0.5, 1.0, 0.5,
                0.5, 0.5, 0.5,
            ],
            0.5, 0.5,
            vec![
                0.3125, 0.375, 0.375,
                0.375, 0.8125, 0.5625,
                0.375, 0.5625, 0.5625,
            ]
        );
    }
}
