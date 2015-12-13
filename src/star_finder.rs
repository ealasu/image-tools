use image::Image;

#[derive(Debug)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

pub struct StarFinder<'a> {
    image: &'a Image
}

impl<'a> StarFinder<'a> {
    pub fn new(image: &'a Image) -> StarFinder {
        StarFinder {
            image: image
        }
    }

    pub fn find(&self) -> Vec<Point> {
        let im = self.image;
        let max = *im.pixels().iter().max().unwrap() as f32;
        let min = *im.pixels().iter().min().unwrap() as f32;
        println!("max: {}", max);
        println!("min: {}", min);

        let average: f32 = im.pixels().iter().map(|&v| v as f32).fold(0f32, |sum, i| sum + i) /
            im.pixels().len() as f32;
        println!("average: {}", average);
        let background = average;

        let threshold = ((max - background) * 0.75 + background) as u16;
        let mut c: u32 = 0;

        for (i, &v) in im.pixels().iter().enumerate() {
            if v >= threshold {
                c += 1;
            }
        }
        println!("c: {}", c);

        Vec::new()
    }
}
