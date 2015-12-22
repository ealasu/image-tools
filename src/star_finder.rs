use std::cmp::max;
use std::ops::Range;
use std::f32;
use image::Image;
use point::Point;


pub type Star = Point<usize>;

pub struct StarFinder<'a> {
    image: &'a Image,
    pos_iter: Range<usize>,
    peak_min: f32,
    peak_max: f32,
    x_min: usize,
    x_max: usize,
    y_min: usize,
    y_max: usize,
}

impl<'a> StarFinder<'a> {
    pub fn new(image: &'a Image) -> StarFinder {
        let min_pixel = image.pixels().iter().fold(f32::MAX, |acc, &v| acc.min(v));
        let max_pixel = image.pixels().iter().fold(f32::MIN, |acc, &v| acc.max(v));
        //println!("max: {}", max);
        //println!("min: {}", min);

        let peak_min = (max_pixel - min_pixel) * 0.8 + min_pixel;
        let peak_max = max_pixel;

        //let average: f32 = image.pixels().iter().map(|&v| v as f32).fold(0f32, |sum, i| sum + i) /
            //image.pixels().len() as f32;
        //println!("average: {}", average);
        //let background = average;

        //let bg_threshold = ((max - background) * 0.00 + background) as u16;
        //let fg_threshold = ((max - background) * 0.75 + background) as u16;
        //println!("bg_threshold: {}", bg_threshold);
        //println!("fg_threshold: {}", fg_threshold);
        let kernel_size = 2;
        let x_padding = max(kernel_size, image.width / 4);
        let y_padding = max(kernel_size, image.height / 4);

        StarFinder {
            image: image,
            pos_iter: 0..image.pixels().len(),
            peak_min: peak_min,
            peak_max: peak_max,
            x_min: x_padding,
            x_max: image.width - x_padding,
            y_min: y_padding,
            y_max: image.height - y_padding,
        }
    }
}

impl<'a> Iterator for StarFinder<'a> {
    type Item = Star;

    fn next(&mut self) -> Option<Star> {
        let image = self.image;
        for pos in &mut self.pos_iter {
            //println!("pos: {} ({},{})", pos, pos % image.width, pos / image.width);
            let pixel = image.pixels()[pos];

            if pixel < self.peak_min {
                continue;
            }
            if pixel > self.peak_max {
                //println!("too bright");
                continue;
            }

            let x = pos % image.width;
            let y = pos / image.width;
            if x < self.x_min ||
               x > self.x_max ||
               y < self.y_min ||
               y > self.y_max {
                continue;
            }
            //println!("pixel: {}", pixel);

            // 1 pixel left, right, above, and below
            let bingo = (y - 1..y + 2).all(|yy| {
                (x - 1..x + 2).all(|xx| {
                    let neighbor = image.at(xx, yy);
                    //println!("n ({},{}): {}", xx, yy, neighbor);
                    if neighbor > pixel {
                        return false;
                    } else if pixel == neighbor {
                        if xx != x || yy != y {
                            if (xx >= x && yy <= y) || (xx > x && yy < y) {
                                return false;
                            }
                        }
                    }
                    true
                })
            });
            if bingo {
                return Some(Star {x: x, y: y});
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;
    use super::*;
    use image::Image;

    #[test]
    fn test_star() {
        let image = Image::load("data/star.tiff");
        let finder = StarFinder::new(&image);
        let stars: Vec<_> = finder.collect();

        assert_eq!(stars, vec![Star {x: 10, y: 7}]);
    }

    #[test]
    fn test_tiny() {
        let image = Image::load("data/tiny.tiff");
        let finder = StarFinder::new(&image);
        let stars: Vec<_> = finder.collect();

        assert_eq!(stars, vec![Star {x: 66, y: 19}, Star {x: 61, y: 41}]);
    }

    #[test]
    #[ignore]
    fn test_small() {
        let image = Image::load("data/small.tiff");
        let finder = StarFinder::new(&image);
        let stars: Vec<_> = finder.collect();

        assert_eq!(stars.len(), 80);
    }

    #[test]
    #[ignore]
    fn test_big() {
        let image = Image::load("data/big-1.tiff");
        let finder = StarFinder::new(&image);
        let stars: Vec<_> = finder.collect();

        assert_eq!(stars.len(), 223);
    }

    #[bench]
    fn bench_star(b: &mut Bencher) {
        let image = Image::load("data/star.tiff");
        b.iter(|| {
            let finder = StarFinder::new(&image);
            let _: Vec<_> = finder.collect();
        });
    }

    #[bench]
    fn bench_setup_start(b: &mut Bencher) {
        let image = Image::load("data/star.tiff");
        b.iter(|| {
            let _ = StarFinder::new(&image);
        });
    }
    
    #[bench]
    fn bench_setup_tiny(b: &mut Bencher) {
        let image = Image::load("data/tiny.tiff");
        b.iter(|| {
            let _ = StarFinder::new(&image);
        });
    }

    #[bench]
    fn bench_tiny(b: &mut Bencher) {
        let image = Image::load("data/tiny.tiff");
        b.iter(|| {
            let finder = StarFinder::new(&image);
            let _: Vec<_> = finder.collect();
        });
    }
}
