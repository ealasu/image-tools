use std::cmp::{min, max};
use std::ops::Range;
use image::Image;
use point::{Point, IPoint};
use spiral::spiral;


#[derive(Debug, PartialEq)]
pub struct Star {
    x: usize,
    y: usize,
    image: Image,
}

pub struct StarFinder<'a> {
    image: &'a Image,
    pos_iter: Range<usize>,
    bg_threshold: u16,
    fg_threshold: u16,
}

impl<'a> StarFinder<'a> {
    pub fn new(image: &'a Image) -> StarFinder {
        let max = *image.pixels().iter().max().unwrap() as f32;
        let min = *image.pixels().iter().min().unwrap() as f32;
        println!("max: {}", max);
        println!("min: {}", min);

        let average: f32 = image.pixels().iter().map(|&v| v as f32).fold(0f32, |sum, i| sum + i) /
            image.pixels().len() as f32;
        println!("average: {}", average);
        let background = average;

        let bg_threshold = ((max - background) * 0.00 + background) as u16;
        let fg_threshold = ((max - background) * 0.75 + background) as u16;
        println!("bg_threshold: {}", bg_threshold);
        println!("fg_threshold: {}", fg_threshold);

        StarFinder {
            image: image,
            pos_iter: 0..image.pixels().len(),
            bg_threshold: bg_threshold,
            fg_threshold: fg_threshold,
        }
    }
}

impl<'a> Iterator for StarFinder<'a> {
    type Item = Star;

    fn next(&mut self) -> Option<Star> {
        let pixels = self.image.pixels();

        // search for a bright pixel
        'outer: for pos in &mut self.pos_iter {
            println!("pos: {} ({},{})", pos, pos % self.image.width, pos / self.image.width);
            if pixels[pos] > self.fg_threshold {
                // found a match
                let v = pixels[pos];

                let center_x = pos % self.image.width;
                let center_y = pos / self.image.width;
                //println!("match: {},{}", x, y);

                // spiral around to determine the full extents of the star
                let min_radius = 2;
                let max_radius = 8;

                let mut left: usize = 0;
                let mut right: usize = 0;
                let mut top: usize = 0;
                let mut bottom: usize = 0;

                'spiral: for (r, mut side_points) in spiral() {
                    println!("r: {}", r);
                    if r > max_radius {
                        // TODO: optimization: block out this square
                        println!("bigger than max radius");
                        continue 'outer;
                    }
                    let mut side_max_v: u16 = 0;
                    for IPoint {x,y} in side_points {
                        if x < 0 || y < 0 || x >= self.image.width as isize || y >= self.image.height as isize {
                            println!("break");
                            break 'spiral;
                        }
                        left = min(left, x as usize);
                        right = max(right, x as usize);
                        top = min(top, y as usize);
                        bottom = max(bottom, y as usize);

                        side_max_v = max(side_max_v, self.image.at(x as usize, y as usize));
                    }
                    if side_max_v < self.bg_threshold {
                        println!("side_max_v: {}", side_max_v);
                        if r < min_radius {
                            // TODO: optimization: block out this square?
                            println!("radius too small");
                            continue 'outer;
                        }
                        // TODO: block out this square
                        return Some(Star {
                            x: left,
                            y: top,
                            image: self.image.crop(left, top, right, bottom),
                        });
                    }
                }

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
