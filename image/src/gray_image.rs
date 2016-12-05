use std::iter::once;
use image::*;
use channel::*;
use magick::*;
use pgm;
use dcraw;


#[derive(Debug)]
pub struct GrayImage<P>(pub Channel<P>);

impl GrayImage<f32> {
    pub fn open(path: &str) -> Self {
        let (width, height, data) = magick_stream(path, "gray");
        GrayImage(Channel::from_data(width, height, data))
    }

    pub fn into_channel(self) -> Channel<f32> {
        self.0
    }

    pub fn open_pgm(path: &str) -> Self {
        let (w, h, data) = pgm::read_from_file(path);
        let data = if let pgm::Format::F32(d) = data { d } else {
            panic!("bad format")
        };
        GrayImage(Channel::from_data(w, h, data))
    }

    pub fn save(&self, path: &str) {
        magick_convert(self.0.pixels(), self.width(), self.height(), "gray", "grayscale", path);
    }

    pub fn save_pgm(&self, path: &str) {
        let data = pgm::Format::F32(self.0.pixels().clone());
        pgm::write_to_file(path, self.width(), self.height(), data);
    }
}

impl GrayImage<u16> {
    pub fn open_raw(path: &str) -> Self {
        let (w, h, data) = dcraw::open_raw(path);
        GrayImage(Channel::from_data(w, h, data))
    }

    pub fn rescale_to_f32(&self) -> GrayImage<f32> {
        // TODO: rescale to 0..1?
        let data = self.0.pixels();
        let mut out: Vec<f32> = Vec::with_capacity(data.len());
        for v in data.iter() {
            out.push(*v as f32);
        }
        GrayImage(Channel::from_data(self.width(), self.height(), out))
    }
}

impl<P> Image<P> for GrayImage<P>
where P: Copy + Clone + Default {
    fn new(width: usize, height: usize) -> Self {
        GrayImage(Channel::new(width, height))
    }

    fn width(&self) -> usize { self.0.width }
    fn height(&self) -> usize { self.0.height }

    fn channels<'a>(&'a self) -> Box<Iterator<Item=&'a Channel<P>> + 'a> {
        Box::new(once(&self.0))
    }

    fn channels_mut<'a>(&'a mut self) -> Box<Iterator<Item=&'a mut Channel<P>> + 'a> {
        Box::new(once(&mut self.0))
    }
}
