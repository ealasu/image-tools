use image::Image;
use projection::Projection;
use remove_background::remove_background;
use align::align;

pub struct ThreeAxisDonuts {
    ref_center: Projection,
    // q2 | q1
    // ---|---
    // q3 | q4
    ref_q1: Projection,
    ref_q2: Projection,
    ref_q3: Projection,
    ref_q4: Projection,
}

const SIZE: usize = 800;
const BG_TILES: usize = 24;

fn fix(mut image: Image<f32>) -> Image<f32> {
    remove_background(&mut image, BG_TILES);
    image
}

impl ThreeAxisDonuts {
    pub fn new(image: &Image<f32>) -> Self {
        ThreeAxisDonuts {
            ref_center: Projection::new(
                &fix(image.center_crop(SIZE, SIZE))),
            ref_q1: Projection::new(
                &fix(image.crop(image.width - SIZE, 0, SIZE, SIZE))),
            ref_q2: Projection::new(
                &fix(image.crop(0, 0, SIZE, SIZE))),
            ref_q3: Projection::new(
                &fix(image.crop(0, image.height - SIZE, SIZE, SIZE))),
            ref_q4: Projection::new(
                &fix(image.crop(image.width - SIZE, image.height - SIZE, SIZE, SIZE))),
        }
    }

    pub fn align(&self, image: &Image<f32>) {
        let sam_center = Projection::new(
            &fix(image.center_crop(SIZE, SIZE)));
        let sam_q1 = Projection::new(
            &fix(image.crop(image.width - SIZE, 0, SIZE, SIZE)));
        let sam_q2 = Projection::new(
            &fix(image.crop(0, 0, SIZE, SIZE)));
        let sam_q3 = Projection::new(
            &fix(image.crop(0, image.height - SIZE, SIZE, SIZE)));
        let sam_q4 = Projection::new(
            &fix(image.crop(image.width - SIZE, image.height - SIZE, SIZE, SIZE)));
        let d_c = align(&self.ref_center, &sam_center);
        let d_q1 = align(&self.ref_q1, &sam_q1) - d_c;
        let d_q2 = align(&self.ref_q2, &sam_q2) - d_c;
        let d_q3 = align(&self.ref_q3, &sam_q3) - d_c;
        let d_q4 = align(&self.ref_q4, &sam_q4) - d_c;
        println!("d_c: {:?}", d_c);
        println!("d_q1: {:?}", d_q1);
        println!("d_q2: {:?}", d_q2);
        println!("d_q3: {:?}", d_q3);
        println!("d_q4: {:?}", d_q4);
    }
}
