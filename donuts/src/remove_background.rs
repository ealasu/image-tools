use image::GrayImage;
use quickersort::sort_floats;

pub fn remove_background(image: &mut GrayImage<f32>, tiles: usize) {
    let max = *image.pixels.iter().max_by(|a,b| a.partial_cmp(b).unwrap()).unwrap();
    let tile_w = image.width / tiles;
    let tile_h = image.height / tiles;
    let mut tile: Vec<f32> = Vec::with_capacity(tile_w * tile_h);
    for tile_x in 0..tiles {
        for tile_y in 0..tiles {
            tile.clear();
            let x1 = tile_x * tile_w;
            let y1 = tile_y * tile_h;
            let x2 = x1 + tile_w;
            let y2 = y1 + tile_h;

            for y in y1..y2 {
                let start = y * image.width;
                tile.extend_from_slice(&image.pixels[start + x1 .. start + x2]);
            }
            sort_floats(&mut tile[..]);
            let med = tile[tile.len() / 2];
            let bg = med + (max - med) * 0.1; // chop out faint stars & noise
            for y in y1..y2 {
                for x in x1..x2 {
                    let pixel = &mut image.pixels[y * image.width + x];
                    *pixel -= bg;
                    if *pixel < 0.0 {
                        *pixel = 0.0;
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use rand::{self, Rng};
    use image::GrayImage;

    #[test]
    fn test() {
        let mut img = GrayImage::open("test/in.jpg");
        img.save("test/in-gray.jpg");
        remove_background(&mut img, 32);
        img.save("test/in-minus-background.jpg");
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        let w = 2000;
        let h = 1000;
        let image = GrayImage {
            width: w,
            height: h,
            pixels: rand::thread_rng().gen_iter().take(w * h).collect()
        };
        b.iter(|| {
            let mut image = image.clone();
            remove_background(&mut image, 16)
        });
    }
}
