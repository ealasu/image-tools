use rgb_bayer::RgbBayer;
use rgb::Rgb;
use image::Image;
use num::Float;

impl<P: Float> Image<RgbBayer<P>> {
    pub fn to_green(&self) -> Image<P> {
        self.map(|p| {
            if p.gc == P::zero() { P::zero() } else { p.g / p.gc }
        })
    }

    pub fn to_green_interpolated(&self) -> Image<P> {
        let mut pixels = Vec::with_capacity(self.width * self.height);
        let four = P::one() + P::one() + P::one() + P::one();
        for y in 0..self.height {
            for x in 0..self.width {
                let p = *self.pixel_at(x, y);
                let gray = if p.gc > P::zero() {
                    p.g
                } else {
                    let left = if x == 0 { P::zero() } else { self.pixel_at(x - 1, y).g };
                    let right = if x == self.width - 1 { P::zero() } else { self.pixel_at(x + 1, y).g };
                    let top = if y == 0 { P::zero() } else { self.pixel_at(x, y - 1).g };
                    let bottom = if y == self.height - 1 { P::zero() } else { self.pixel_at(x, y + 1).g };
                    (left + right + top + bottom) / four
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

    pub fn to_rgb(&self) -> Image<Rgb<P>> {
        self.map(|p| {
            Rgb {
                r: if p.rc == P::zero() { P::zero() } else { p.r / p.rc },
                g: if p.gc == P::zero() { P::zero() } else { p.g / p.gc },
                b: if p.bc == P::zero() { P::zero() } else { p.b / p.bc },
            }
        })
    }

    pub fn holes(&self) -> Image<Rgb<P>> {
        let two = P::one() + P::one();
        self.map(|p| {
            Rgb {
                r: p.rc,
                g: p.gc / two,
                b: p.bc,
            }
        })
    }

    pub fn correct_white_balance(&self) -> Self {
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
    pub fn avg(&self) -> (P, P, P) {
        let mut sum_r = P::zero();
        let mut count_r = P::zero();
        let mut sum_g = P::zero();
        let mut count_g = P::zero();
        let mut sum_b = P::zero();
        let mut count_b = P::zero();
        for p in self.pixels.iter() {
            sum_r   = sum_r     + p.r * p.rc;
            count_r = count_r   + p.rc;
            sum_g   = sum_g     + p.g * p.gc;
            count_g = count_g   + p.gc;
            sum_b   = sum_b     + p.b * p.bc;
            count_b = count_b   + p.bc;
        }
        let avg_r = sum_r / count_r;
        let avg_g = sum_g / count_g;
        let avg_b = sum_b / count_b;
        (avg_r, avg_g, avg_b)
    }
}
