use std::cmp::{min, max};
use std::ops::Range;
use image::Image;
use point::Point;
use spiral::spiral;


pub type Star = Point<f32>;

pub struct StarFinder<'a> {
    image: &'a Image,
    pos_iter: Range<usize>,
    peak_min: f32,
    peak_max: f32,
}

impl<'a> StarFinder<'a> {
    pub fn new(image: &'a Image) -> StarFinder {
        //let max = *image.pixels().iter().max().unwrap() as f32;
        //let min = *image.pixels().iter().min().unwrap() as f32;
        //println!("max: {}", max);
        //println!("min: {}", min);

        //let average: f32 = image.pixels().iter().map(|&v| v as f32).fold(0f32, |sum, i| sum + i) /
            //image.pixels().len() as f32;
        //println!("average: {}", average);
        //let background = average;

        //let bg_threshold = ((max - background) * 0.00 + background) as u16;
        //let fg_threshold = ((max - background) * 0.75 + background) as u16;
        //println!("bg_threshold: {}", bg_threshold);
        //println!("fg_threshold: {}", fg_threshold);

        StarFinder {
            image: image,
            pos_iter: 0..image.pixels().len(),
            peak_min: 0f32,
            peak_max: 1f32,
        }
    }
}

impl<'a> Iterator for StarFinder<'a> {
    type Item = Star;

    fn next(&mut self) -> Option<Star> {
        let image = self.image;
        for pos in &mut self.pos_iter {
            println!("pos: {} ({},{})", pos, pos % image.width, pos / image.width);
            let pixel = image.pixels()[pos];

            if pixel < self.peak_min || pixel > self.peak_max {
                continue;
            }

            let x = pos % image.width;
            let y = pos / image.width;
            if x < 2 || x > image.width - 2 || y < 2 || y > image.height - 2 {
                continue;
            }

            let bingo = (y - 1..y + 1).all(|yy| {
                (x - 1..x + 1).all(|xx| {
                    (yy == y && xx == x) || image.at(xx, yy) < pixel
                })
            });
            if bingo {
                return Some(Star {x: x as f32, y: y as f32});
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::Image;

    #[test]
    fn test() {
        let image = Image::load("data/star.tiff");
        println!("input: {:?}", image);

        let finder = StarFinder::new(&image);
        let stars: Vec<_> = finder.collect();

        assert_eq!(stars, vec![]);
    }
}
