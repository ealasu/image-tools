use image::Image;
use projection::Projection;
use remove_background::remove_background;
use align::align;
use geom::*;

pub struct ThreeAxisDonuts {
    center: Vector<f32>,
    q1: Vector<f32>,
    q2: Vector<f32>,
    q3: Vector<f32>,
    q4: Vector<f32>,
    ref_center: Projection,
    ref_center_small: Projection,
    // q2 | q1
    // ---|---
    // q3 | q4
    ref_q1: Projection,
    ref_q2: Projection,
    ref_q3: Projection,
    ref_q4: Projection,
}

const N: usize = 300;
const SIZE: usize = 1200;
const BG_TILES: usize = 1;
const N_SMALL: usize = 10;
const SIZE_SMALL: usize = 80;
//const BG_TILES_SMALL: usize = 4;
const MARGIN: usize = 300;

fn fix(mut image: Image<f32>) -> Image<f32> {
    remove_background(&mut image, BG_TILES);
    image
}

impl ThreeAxisDonuts {
    pub fn new(image: &Image<f32>) -> Self {
        ThreeAxisDonuts {
            center: Vector {
                x: image.width as f32 / 2.0,
                y: image.height as f32 / 2.0,
            },
            q1: Vector {
                x: (image.width - MARGIN - SIZE / 2) as f32,
                y: (MARGIN + SIZE / 2) as f32,
            },
            q2: Vector {
                x: (MARGIN + SIZE / 2) as f32,
                y: (MARGIN + SIZE / 2) as f32,
            },
            q3: Vector {
                x: (MARGIN + SIZE / 2) as f32,
                y: (image.height - MARGIN - SIZE / 2) as f32,
            },
            q4: Vector {
                x: (image.width - MARGIN - SIZE / 2) as f32,
                y: (image.height - MARGIN - SIZE / 2) as f32,
            },
            ref_center: Projection::new(
                &fix(image.center_crop(SIZE, SIZE))),
            ref_center_small: Projection::new(
                &fix(image.center_crop(SIZE_SMALL, SIZE_SMALL))),
            ref_q1: Projection::new(
                &fix(image.crop(
                        image.width - SIZE - MARGIN,
                        MARGIN,
                        SIZE, SIZE))),
            ref_q2: Projection::new(
                &fix(image.crop(
                        MARGIN,
                        MARGIN,
                        SIZE, SIZE))),
            ref_q3: Projection::new(
                &fix(image.crop(
                        MARGIN,
                        image.height - SIZE - MARGIN,
                        SIZE, SIZE))),
            ref_q4: Projection::new(
                &fix(image.crop(
                        image.width - SIZE - MARGIN,
                        image.height - SIZE - MARGIN,
                        SIZE, SIZE))),
        }
    }

    pub fn align(&self, image: &Image<f32>) -> Matrix3x3<f32> {
        let sam_center = Projection::new(
            &fix(image.center_crop(SIZE, SIZE)));
        let d_c = align(&self.ref_center, &sam_center, N);
        println!("estimate d_c: {:?}", d_c);
        let d_small = d_c.floor();
        let sam_center_small = Projection::new(
            &fix(image.crop(
                image.width / 2 - SIZE_SMALL / 2 - d_small.x as usize,
                image.height / 2 - SIZE_SMALL / 2 - d_small.y as usize,
                SIZE_SMALL, SIZE_SMALL)));
        let d_c = d_small + align(&self.ref_center_small, &sam_center_small, N_SMALL);
        println!("precise d_c: {:?}", d_c);

        let sam_q1 = Projection::new(
            &fix(image.crop(
                    image.width - SIZE - MARGIN - d_c.x as usize,
                    MARGIN - d_c.y as usize,
                    SIZE, SIZE)));
        let sam_q2 = Projection::new(
            &fix(image.crop(
                    MARGIN - d_c.x as usize,
                    MARGIN - d_c.y as usize,
                    SIZE, SIZE)));
        let sam_q3 = Projection::new(
            &fix(image.crop(
                    MARGIN - d_c.x as usize,
                    image.height - SIZE - MARGIN - d_c.y as usize,
                    SIZE, SIZE)));
        let sam_q4 = Projection::new(
            &fix(image.crop(
                    image.width - SIZE - MARGIN - d_c.x as usize,
                    image.height - SIZE - MARGIN - d_c.y as usize,
                    SIZE, SIZE)));

        let d_q1 = align(&self.ref_q1, &sam_q1, N);
        let d_q2 = align(&self.ref_q2, &sam_q2, N);
        let d_q3 = align(&self.ref_q3, &sam_q3, N);
        let d_q4 = align(&self.ref_q4, &sam_q4, N);

        println!("d_q1: {:?}", d_q1);
        println!("d_q2: {:?}", d_q2);
        println!("d_q3: {:?}", d_q3);
        println!("d_q4: {:?}", d_q4);

        let q1_a = (self.q1 + d_q1 - self.center).angle() - (self.q1 - self.center).angle();
        let q2_a = (self.q2 + d_q2 - self.center).angle() - (self.q2 - self.center).angle();
        let q3_a = (self.q3 + d_q3 - self.center).angle() - (self.q3 - self.center).angle();
        let q4_a = (self.q4 + d_q4 - self.center).angle() - (self.q4 - self.center).angle();
        println!("angles: {},{},{},{}", q1_a, q2_a, q3_a, q4_a);
        let angle = (q1_a + q2_a + q3_a + q4_a) / 4.0;

        //Matrix3x3::identity()

        Matrix3x3::translation(image.width as f32 / 2.0 - d_c.x, image.height as f32 / 2.0 - d_c.y) *
        //Matrix3x3::translation(image.width as f32 / 2.0 , image.height as f32 / 2.0) *
        Matrix3x3::rotation(angle) *
        Matrix3x3::translation(-(image.width as f32) / 2.0, -(image.height as f32) / 2.0)
    }
}
