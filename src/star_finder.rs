use image::Image;

#[derive(Debug)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug)]
pub struct Star<'a> {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    data: &'a [u16],
}

pub struct StarFinder<'a> {
    image: &'a Image,
    pos: usize,
    threshold: u16,
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

        let threshold = ((max - background) * 0.75 + background) as u16;

        StarFinder {
            image: image,
            pos: 0,
            threshold: threshold ,
        }
    }
}

impl<'a> Iterator for StarFinder<'a> {
    type Item = Star<'a>;

    fn next(&mut self) -> Option<Star<'a>> {
        let pixels = self.image.pixels();

        // search for a bright pixel
        loop {
            if pixels[self.pos] > self.threshold {
                // found a match
                let v = pixels[self.pos];

                let x = self.pos % self.image.width;
                let y = self.pos / self.image.width;
                println!("match: {},{}", x, y);
            }
            self.pos += 1;
            if self.pos >= pixels.len() {
                return None;
            }
        }
    }
}
