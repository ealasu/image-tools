use image::Image;
use statistical::median;

pub fn remove_background(image: &mut Image, tiles: usize) {
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
            tile.extend(
                (y1..y2).flat_map(|y| {
                    let start = y * image.width;
                    image.pixels[start + x1 .. start + x2].iter()
                }));
            let med = median(&tile[..]);
            for y in y1..y2 {
                for x in x1..x2 {
                    image.pixels[y * image.width + x] -= med;
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

    #[test]
    fn test() {
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        let w = 2000;
        let h = 1000;
        let image = Image {
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
