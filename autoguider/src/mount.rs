use pos::Vector;
use std::thread;
use std::time::Duration;

pub struct Mount {
}

impl Mount {
    pub fn new() -> Self {
        Mount {}
    }

    pub fn slew(&mut self, amount_pixels: Vector) {
        let pixel_size_um: f32 = 6.54; // for Canon 6D
        let focal_length_mm: f32 = 200.0;
        let pixel_size_arcsec: f32 = pixel_size_um / focal_length_mm * 206.3;
        let amount_arcsecs = Vector {
            x: amount_pixels.x * pixel_size_arcsec,
            y: amount_pixels.y * pixel_size_arcsec,
        };
        info!("slewing {:?} arcsecs", amount_arcsecs);

        thread::sleep(Duration::from_secs(1));
    }
}
