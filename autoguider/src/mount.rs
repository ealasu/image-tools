use pos::Vector;
use std::thread;
use std::time::Duration;
use scope_client::Client;

pub struct Mount {
    client: Client,
}

fn arcsec_to_step(v: f32) -> i32 {
    let v = v as f64;
    let secs_per_step = 69044.0 / 1000000.0;
    let arcsec_per_sec = (360.0*60.0*60.0) / (23.9344699*60.0*60.0);
    let res: f64 = v / arcsec_per_sec / secs_per_step;
    res as i32
}

impl Mount {
    pub fn new() -> Self {
        Mount {
            client: Client::new(),
        }
    }
    
    // TODO: auto calibration

    pub fn slew(&mut self, amount_pixels: Vector) {
        let pixel_size_um = 6.54; // for Canon 6D
        let focal_length_mm = 200.0;
        let pixel_size_arcsec = pixel_size_um / focal_length_mm * 206.3;

        let dec = arcsec_to_step(amount_pixels.x * pixel_size_arcsec / 2.5);
        let ra = arcsec_to_step(amount_pixels.y * pixel_size_arcsec / 1.6);

        info!("slew_by ra: {}, dec: {}", ra, dec);
        self.client.slew_by(ra, dec).unwrap();
        thread::sleep(Duration::from_secs(2));
        let msg = self.client.read_msg();
        info!("msg from scope: {:?}", msg);
    }

}
