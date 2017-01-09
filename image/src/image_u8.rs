use image::Image;
use rgb::Rgb;

impl Image<u8> {
    pub fn to_rgb(&self) -> Image<Rgb<u8>> {
        self.map(|&p| Rgb { r: p, g: p, b: p })
    }
}
