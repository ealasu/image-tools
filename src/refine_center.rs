use image::Image;
use point::Point;


pub fn refine_center(image: &Image, start: Point<usize>) -> Point<f32> {
    
}


#[cfg(test)]
mod tests {
    use test::Bencher;
    use super::*;
    use image::Image;

    #[test]
    fn test_star() {
        let image = Image::load("data/star.tiff");
        let start = Point {x: 10, y: 7};
        let center = refine_center(&image, start);

        assert_eq!(center, Point {x: 10.0, y: 7.0});
    }
}
