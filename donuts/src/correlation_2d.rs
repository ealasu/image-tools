use cross_range::cross_range;
use geom::Vector;
use image::Image;
use simd::x86::avx::{f32x8, LowHigh128};
use correlation::{pos_of_max, interpolate_peak_pos};

fn row_slice(img: &Image<f32>, n: usize, x: isize, y: isize) -> &[f32] {
    let top = (y + n as isize) as usize;
    let left = (x + n as isize) as usize;
    let right = (-x + n as isize) as usize;
    img.row_slice(top, left, right)
}

fn corr(a: &[f32], b: &[f32]) -> f32 {
    let mut res = 0.0;
    for (a, b) in a.iter().zip(b.iter()) {
        res += a * b;
    }
    res
}

fn corr8(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    let mut sum = f32x8::splat(0.0);
    let mut i = 0;
    let len = a.len();
    while i < len & !7 {
        let a = f32x8::load(a, i);
        let b = f32x8::load(b, i);
        sum = sum + a * b;
        i += 8;
    }
    let sum = sum.low() + sum.high();
    let mut sum = sum.extract(0) + sum.extract(1) + sum.extract(2) + sum.extract(3);
    while i < len {
        sum = sum + a[i] * b[i];
        i += 1;
    }
    sum
}

pub fn correlation_2d(ref_img: &Image<f32>, sample_img: &Image<f32>, n: usize) -> Image<f32> {
    let mut res = Image::new(n * 4 + 1, n * 4 + 1);
    let h = ref_img.height - n * 2;
    for (y_corr, (y_r, y_s)) in cross_range(n).enumerate() {
        //println!("y_r: {} y_s: {}", y_r, y_s);
        for (x_corr, (x_r, x_s)) in cross_range(n).enumerate() {
            //println!(" x_r: {} x_s: {}", x_r, x_s);
            let mut sum = 0.0;
            for y in 0..h as isize {
                let row_r = row_slice(ref_img, n, x_r, y_r + y);
                let row_s = row_slice(sample_img, n, x_s, y_s + y);
                sum += corr8(row_r, row_s);
            }
            *res.pixel_at_mut(x_corr, y_corr) += sum;
        }
    }
    res
}

pub fn correlation_peak(corr: &Image<f32>) -> Vector<f32> {
    let n_corr = ((corr.width - 1) / 2) as f32;
    let peak_pos = pos_of_max(&corr.pixels[..]);
    let peak_pos_x = peak_pos % corr.width;
    let peak_pos_y = peak_pos / corr.width;
    if peak_pos_x == 0 || peak_pos_x == corr.width - 1 ||
        peak_pos_y == 0 || peak_pos_y == corr.height - 1 {
        return Vector {
            x: -(peak_pos_x as f32),
            y: -(peak_pos_y as f32),
        };
    }
    Vector {
        x: -(interpolate_peak_pos([
            *corr.pixel_at(peak_pos_x - 1, peak_pos_y),
            *corr.pixel_at(peak_pos_x, peak_pos_y),
            *corr.pixel_at(peak_pos_x + 1, peak_pos_y),
        ], peak_pos_x as f32) - n_corr),
        y: -(interpolate_peak_pos([
            *corr.pixel_at(peak_pos_x, peak_pos_y - 1),
            *corr.pixel_at(peak_pos_x, peak_pos_y),
            *corr.pixel_at(peak_pos_x, peak_pos_y + 1),
        ], peak_pos_y as f32) - n_corr),
    }
}

pub fn align(ref_img: &Image<f32>, sample_img: &Image<f32>, n: usize) -> Vector<f32> {
    let corr = correlation_2d(ref_img, sample_img, n);
    correlation_peak(&corr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Image, Rgb};
    use test::Bencher;
    use rand::{self, Rng};

    #[test]
    fn test() {
        let ref_img = Image {
            width: 3,
            height: 3,
            pixels: vec![
                0.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 0.0,
            ],
        };
        let sample_img = Image {
            width: 3,
            height: 3,
            pixels: vec![
                0.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 0.0,
            ],
        };
        let corr = correlation_2d(&ref_img, &sample_img, 1);
        let expected = [
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
        ];
        assert_eq!(&corr.pixels[..], &expected[..]);
    }

    #[test]
    fn test_offset() {
        let ref_img = Image {
            width: 3,
            height: 3,
            pixels: vec![
                0.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 0.0,
            ],
        };
        let sample_img = Image {
            width: 3,
            height: 3,
            pixels: vec![
                0.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
                0.0, 0.0, 1.0,
            ],
        };
        let corr = correlation_2d(&ref_img, &sample_img, 1);
        let expected = [
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
        ];
        assert_eq!(&corr.pixels[..], &expected[..]);
    }

    #[test]
    fn test_offset_2() {
        let ref_img = Image {
            width: 5,
            height: 5,
            pixels: vec![
                0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0,
            ],
        };
        let sample_img = Image {
            width: 5,
            height: 5,
            pixels: vec![
                0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        };
        let corr = correlation_2d(&ref_img, &sample_img, 1);
        let expected = [
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 1.0,
        ];
        assert_eq!(&corr.pixels[..], &expected[..]);
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        let s = 1000;
        let make_image = || {
            Image {
                width: s,
                height: s,
                pixels: rand::thread_rng().gen_iter().take(s * s).collect()
            }
        };
        let ref_img = make_image();
        let sam_img = make_image();
        b.iter(|| {
            correlation_2d(&ref_img, &sam_img, 30)
        });
    }

    #[test]
    fn test_img() {
        let ref_img = Image::<Rgb<u8>>::open_jpeg_file("test/a.jpg")
            .to_f32()
            .remove_background(1.0)
            .to_gray()
            .center_crop(900, 900);
        let sam_img = Image::<Rgb<u8>>::open_jpeg_file("test/b.jpg")
            .to_f32()
            .remove_background(1.0)
            .to_gray()
            .center_crop(900, 900);

        let corr = correlation_2d(&ref_img, &sam_img, 100);
        corr.save("res.jpg");
    }
}
