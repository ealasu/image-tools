use std::f32;
use image::*;
use point::Point;


pub fn refine_star_center(image: &Channel<f32>, start: Point<usize>, aperture: usize) -> Point<f32> {
    assert!(aperture <= start.x);
    assert!(aperture <= image.width - start.x);
    assert!(aperture <= start.y);
    assert!(aperture <= image.height- start.y);

    let x_data: Vec<f32> = (start.x - aperture..start.x + aperture).map(|x| {
        (start.y - aperture..start.y + aperture).map(|y| {
            image.at(x, y)
        }).fold(0f32, |acc, v| acc + v)
    }).collect();
    let y_data: Vec<f32> = (start.y - aperture..start.y + aperture).map(|y| {
        (start.x - aperture..start.x + aperture).map(|x| {
            image.at(x, y)
        }).fold(0f32, |acc, v| acc + v)
    }).collect();
    let x = centroid(&x_data);
    let y = centroid(&y_data);

    Point {
        x: (start.x - aperture) as f32 + x,
        y: (start.y - aperture) as f32 + y,
    }
}

pub fn centroid(data: &[f32]) -> f32 {
    let iterations = 10;
    let minimum = data.iter().fold(f32::MAX, |acc, &v| acc.min(v));

    let mut center = 0f32;
    let mut delta = 0f32;
    for _ in 0..iterations {
        center += delta;
        let (sum1, sum2) = data.iter().enumerate().fold((0f32, 0f32), |(sum1, sum2), (position, pixel)| {
            let diff = (pixel - minimum).max(0f32);
            (sum1 + (position as f32 - center) * diff, sum2 + diff)
        });
        delta = sum1 / sum2;
    }

    center
}


#[cfg(test)]
mod tests {
    use test::Bencher;
    use super::*;
    use image::Image;
    use point::Point;

    #[test]
    fn test_centroid_1() {
        let data = vec![0f32, 1f32, 2f32, 1f32, 0f32];
        let c = centroid(&data);
        assert_eq!(c, 2f32);
    }

    #[test]
    fn test_centroid_2() {
        let data = vec![0f32, 1f32, 1f32, 0f32];
        let c = centroid(&data);
        assert_eq!(c, 1.5f32);
    }

    #[test]
    fn test_centroid_3() {
        let data = vec![0.02f32, 0.01f32, 2f32, 2f32, 0.02f32, 0.02f32];
        let c = centroid(&data);
        assert_eq!(c, 2.5037405f32);
    }

    #[bench]
    fn bench_centroid(b: &mut Bencher) {
        let data = vec![0f32, 1f32, 2f32, 3f32, 2f32, 1f32, 0f32];
        b.iter(|| {
            centroid(&data)
        });
    }

    #[test]
    fn test_star_1() {
        let image = GrayImage::open("data/star.tiff").into_channel();
        let start = Point {x: 10, y: 7};
        let center = refine_star_center(&image, start, 7);

        assert_eq!(center, Point {x: 9.839535, y: 7.786135});
    }

    #[test]
    fn test_star_2() {
        let image = GrayImage::open("data/star.tiff").into_channel();
        let start = Point {x: 10, y: 7};
        let center = refine_star_center(&image, start, 4);

        assert_eq!(center, Point {x: 9.890924, y: 7.656212});
    }

    #[test]
    fn test_star_off_center() {
        let image = GrayImage::open("data/star.tiff").into_channel();
        let start = Point {x: 9, y: 6};
        let center = refine_star_center(&image, start, 5);

        assert_eq!(center, Point {x: 9.866457, y: 7.5725994});
    }

    #[bench]
    fn bench_star(b: &mut Bencher) {
        let image = GrayImage::open("data/star.tiff").into_channel();
        b.iter(|| {
            let start = Point {x: 10, y: 7};
            refine_star_center(&image, start, 7)
        });
    }
}
