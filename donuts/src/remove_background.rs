use image::Image;
use quickersort::sort_floats;

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

            for y in y1..y2 {
                let start = y * image.width;
                tile.extend_from_slice(&image.pixels[start + x1 .. start + x2]);
            }
            sort_floats(&mut tile[..]);
            let med = tile[tile.len() / 2];
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

    //#[bench]
    //fn median(b: &mut Bencher) {
        //use statistical::median;
        //let v: Vec<f32> = rand::thread_rng().gen_iter().take(100*100).collect();
        //b.iter(|| {
            //let v = v.clone();
            //median(&v[..])
        //});
    //}
}
