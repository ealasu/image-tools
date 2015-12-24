use std::cmp::*;
use std::collections::BTreeMap;
use image::*;
use point::*;


pub fn resample(image: &Channel, x: f32, y: f32) -> f32 {
    let mut src_val = 0f32;
    let dx = x.ceil() - x;
    let dy = y.ceil() - y;
    let dxp = 1.0 - dx;
    let dyp = 1.0 - dy;

    let sw = dx * dyp;
    let nw = dx * dy;
    let ne = dxp * dy;
    let se = dxp * dyp;

    let e_x = x.ceil() as isize;
    let s_y = y.ceil() as isize;
    let w_x = e_x - 1;
    let n_y = s_y - 1;

    let w = image.width as isize;
    let h = image.height as isize;

    if n_y >= 0 && n_y < h && w_x >= 0 && w_x < w {
        src_val += image.at(w_x as usize, n_y as usize) * nw;
    }
    if n_y >= 0 && n_y < h && e_x >= 0 && e_x < w {
        src_val += image.at(e_x as usize, n_y as usize) * ne;
    }
    if s_y >= 0 && s_y < h && e_x >= 0 && e_x < w {
        src_val += image.at(e_x as usize, s_y as usize) * se;
    }
    if s_y >= 0 && s_y < h && w_x >= 0 && w_x < w {
        src_val += image.at(w_x as usize, s_y as usize) * sw;
    }

    src_val
}


pub struct ImageStack {
    image: Image,
    count: usize,
}

impl ImageStack {
    pub fn new(width: usize, height: usize, channel_count: usize) -> ImageStack {
        let channels = (0..channel_count).map(|_| Channel::new(width, height)).collect();
        ImageStack {
            image: Image::new(channels),
            count: 0,
        }
    }

    pub fn add(&mut self, image: &Image, transform: Vector) {
        for (dst, src) in self.image.channels.iter_mut().zip(image.channels.iter()) {
            for y in 0..dst.height {
                for x in 0..dst.width {
                    let src_pos = Point {x: x as f32, y: y as f32} - transform;
                    *dst.at_mut(x, y) += resample(src, src_pos.x, src_pos.y);
                }
            }
        }
        self.count += 1;
    }

    pub fn to_image(mut self) -> Image {
        let d = self.count as f32;
        for c in self.image.channels.iter_mut() {
            for pixel in c.pixels_mut() {
                *pixel /= d;
            }
        }
        self.image
    }
}

pub fn stack(images: &BTreeMap<String, Vector>) {
    // calculate dimensions
    let d = images.iter().map(|(filename, &tx)| {
        let (width, height) = Image::identify(&filename);
        let top = tx.y as isize;
        let bottom = height as isize + tx.y as isize;
        let left = tx.x as isize;
        let right = width as isize + tx.x as isize;
        (top, right, bottom, left)
    }).collect::<Vec<_>>();
    let right = d.iter().map(|&(_, right, _, _)| right).min().unwrap();
    let left =  d.iter().map(|&(_, _, _, left)| left).max().unwrap();
    let width = max(0, right - left) as usize;
    let bottom = d.iter().map(|&(_, _, bottom, _)| bottom).min().unwrap();
    let top = d.iter().map(|&(top, _, _, _)| top).max().unwrap();
    let height = max(0, bottom - top) as usize;
    let stack_tx = Vector {x: -(left as f32), y: -(top as f32)};
    assert!(width > 0);
    assert!(height > 0);

    // stack
    let mut stack = ImageStack::new(width, height, 1);
    for (filename, &tx) in images.iter() {
        let image = Image::open_gray(filename);
        stack.add(&image, tx + stack_tx);
    }
    let image = stack.to_image();

    // save
    println!("res: {:?}", image);
    //let res = image::Image::open("data/big-1-c.tiff");
    image.save_gray("data/out.tiff");
}

#[cfg(test)]
mod tests {
    use test::Bencher;
    use super::*;
    use point::*;
    use image::Image;

    #[test]
    fn test_1() {
        let image = Image::from_data(3, 3, vec![
            0.5, 0.5, 0.5,
            0.5, 1.0, 0.5,
            0.5, 0.5, 0.5,
        ]);
        let v = resample(&image, 1.0, 1.0);
        assert_eq!(v, 1.0);
    }

    #[test]
    fn test_2() {
        let image = Image::from_data(3, 3, vec![
            0.5, 0.5, 0.5,
            0.5, 1.0, 0.5,
            0.5, 0.5, 0.5,
        ]);
        let v = resample(&image, 0.75, 0.75);
        assert_eq!(v, (0.75 * 0.75 * 1.0) + (0.75 * 0.25 * 2.0 * 0.5) + (0.25 * 0.25 * 0.5));
    }

    #[test]
    fn test_edge() {
        let image = Image::from_data(3, 3, vec![
            0.5, 0.5, 0.5,
            0.5, 1.0, 0.5,
            0.5, 0.5, 0.5,
        ]);
        let v = resample(&image, -0.75, -0.75);
        assert_eq!(v, 0.25 * 0.25 * 0.5);
    }

    #[bench]
    fn bench_resample(b: &mut Bencher) {
        let image = Image::from_data(3, 3, vec![
            0.5, 0.5, 0.5,
            0.5, 1.0, 0.5,
            0.5, 0.5, 0.5,
        ]);
        b.iter(|| {
            resample(&image, 1.0, 1.0);
        });
    }

    #[test]
    fn test_stack_1() {
        let image1 = Image::from_data(3, 3, vec![
            0.5, 0.5, 0.5,
            0.5, 1.0, 0.5,
            0.5, 0.5, 0.5,
        ]);
        let mut stacker = ImageStack::new(3, 3);
        stacker.add(&image1, Vector {x: 0.0, y: 0.0});
        assert_eq!(*stacker.to_image().pixels(), vec![
            0.5, 0.5, 0.5,
            0.5, 1.0, 0.5,
            0.5, 0.5, 0.5,
        ]);
    }

    #[test]
    fn test_stack_2() {
        let image1 = Image::from_data(3, 3, vec![
            0.5, 0.5, 0.5,
            0.5, 1.0, 0.5,
            0.5, 0.5, 0.5,
        ]);
        let mut stacker = ImageStack::new(3, 3);
        stacker.add(&image1, Vector {x: 0.0, y: 0.0});
        stacker.add(&image1, Vector {x: 0.0, y: 0.0});
        assert_eq!(*stacker.to_image().pixels(), vec![
            0.5, 0.5, 0.5,
            0.5, 1.0, 0.5,
            0.5, 0.5, 0.5,
        ]);
    }

    #[test]
    fn test_stack_3() {
        let image1 = Image::from_data(3, 3, vec![
            0.5, 0.5, 0.5,
            0.5, 1.0, 0.5,
            0.5, 0.5, 0.5,
        ]);
        let mut stacker = ImageStack::new(3, 3);
        stacker.add(&image1, Vector {x: 0.0, y: 0.0});
        stacker.add(&image1, Vector {x: 0.5, y: 0.5});
        assert_eq!(*stacker.to_image().pixels(), vec![
            0.3125, 0.375, 0.375,
            0.375, 0.8125, 0.5625,
            0.375, 0.5625, 0.5625,
        ]);
    }
}
