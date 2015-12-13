use image::Image;

#[derive(Debug)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

pub struct StarFinder {
    image: Image
}

impl StarFinder {
    pub fn new(image: Image) -> StarFinder {
        StarFinder {
            image: image
        }
    }

    pub fn find(&self) -> Vec<Point> {
        let im = &self.image;
        println!("max: {}", im.pixels().iter().max().unwrap());
        println!("min: {}", im.pixels().iter().min().unwrap());

        let average: f64 = im.pixels().iter().map(|&v| v as f64).fold(0f64, |sum, i| sum + i) /
            im.pixels().len() as f64;
        println!("average: {}", average);

        Vec::new()
    }
}
