use rgb_bayer::RgbBayer;
use rgb::Rgb;
use image::Image;

impl Image<RgbBayer> {
    pub fn to_green(&self) -> Image<f32> {
        self.map(|p| {
            if p.gc == 0.0 { 0.0 } else { p.g / p.gc }
        })
    }

    pub fn to_green_interpolated(&self) -> Image<f32> {
        let mut pixels = Vec::with_capacity(self.width * self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let p = *self.pixel_at(x, y);
                let gray = if p.gc > 0.0 {
                    p.g
                } else {
                    let left = if x == 0 { 0.0 } else { self.pixel_at(x - 1, y).g };
                    let right = if x == self.width - 1 { 0.0 } else { self.pixel_at(x + 1, y).g };
                    let top = if y == 0 { 0.0 } else { self.pixel_at(x, y - 1).g };
                    let bottom = if y == self.height - 1 { 0.0 } else { self.pixel_at(x, y + 1).g };
                    (left + right + top + bottom) / 4.0
                };
                pixels.push(gray);
            }
        }
        Image {
            width: self.width,
            height: self.height,
            pixels: pixels,
        }
    }

    pub fn to_rgb(&self) -> Image<Rgb<f32>> {
        self.map(|p| {
            Rgb {
                r: if p.rc == 0.0 { 0.0 } else { p.r / p.rc },
                g: if p.gc == 0.0 { 0.0 } else { p.g / p.gc },
                b: if p.bc == 0.0 { 0.0 } else { p.b / p.bc },
            }
        })
    }

    pub fn holes(&self) -> Image<Rgb<f32>> {
        self.map(|p| {
            Rgb {
                r: p.rc,
                g: p.gc / 2.0,
                b: p.bc,
            }
        })
    }

    pub fn correct_white_balance(&self) -> Image<RgbBayer> {
        let (avg_r, avg_g, avg_b) = self.avg();
        let m_r = avg_g / avg_r;
        let m_b = avg_g / avg_b;
        self.map(|p| {
            RgbBayer {
                r: p.r * m_r,
                rc: p.rc,
                g: p.g,
                gc: p.gc,
                b: p.b * m_b,
                bc: p.bc,
            }
        })
    }

    /// Computes the average of each component separately.
    /// Returns a tuple of `(red_avg, green_avg, blue_avg)`.
    pub fn avg(&self) -> (f32, f32, f32) {
        let mut sum_r = 0.0;
        let mut count_r = 0.0;
        let mut sum_g = 0.0;
        let mut count_g = 0.0;
        let mut sum_b = 0.0;
        let mut count_b = 0.0;
        for p in self.pixels.iter() {
            sum_r += p.r * p.rc;
            count_r += p.rc;
            sum_g += p.g * p.gc;
            count_g += p.gc;
            sum_b += p.b * p.bc;
            count_b += p.bc;
        }
        let avg_r = sum_r / count_r;
        let avg_g = sum_g / count_g;
        let avg_b = sum_b / count_b;
        (avg_r, avg_g, avg_b)
    }
}
