use std::iter::once;
use convert::*;
use channel::*;
use image::*;
use magick::*;


#[repr(C)]
#[derive(Copy, Clone)]
pub struct Rgb {
    r: f32,
    g: f32,
    b: f32,
}

#[derive(Debug)]
pub struct RgbImage<P> {
    pub r: Channel<P>,
    pub g: Channel<P>,
    pub b: Channel<P>,
}

impl RgbImage<f32> {
    pub fn open(path: &str) -> Self {
        let (width, height, data) = magick_stream(path, "rgb");
        let data: Vec<Rgb> = convert_vec(data);
        assert_eq!(data.len(), width * height);
        let r = data.iter().map(|&p| p.r).collect();
        let g = data.iter().map(|&p| p.r).collect();
        let b = data.iter().map(|&p| p.r).collect();
        RgbImage {
            r: Channel::from_data(width, height, r),
            g: Channel::from_data(width, height, g),
            b: Channel::from_data(width, height, b),
        }
    }

    pub fn save(&self, path: &str) {
        let rgb = (0..self.width() * self.height()).map(|i| {
            Rgb {
                r: self.r.pixels()[i],
                g: self.g.pixels()[i],
                b: self.b.pixels()[i],
            }
        }).collect::<Vec<_>>();
        let data = convert_vec(rgb);
        magick_convert(&data, self.width(), self.height(), "rgb", "truecolor", path);
    }
}

impl<P> Image<P> for RgbImage<P>
where P: Copy + Clone + Default {
    fn new(width: usize, height: usize) -> Self {
        RgbImage {
            r: Channel::new(width, height),
            g: Channel::new(width, height),
            b: Channel::new(width, height),
        }
    }

    fn width(&self) -> usize { self.r.width }
    fn height(&self) -> usize { self.r.height }

    fn channels<'a>(&'a self) -> Box<Iterator<Item=&'a Channel<P>> + 'a> {
        Box::new(once(&self.r)
                 .chain(once(&self.g))
                 .chain(once(&self.b)))
    }

    fn channels_mut<'a>(&'a mut self) -> Box<Iterator<Item=&'a mut Channel<P>> + 'a> {
        Box::new(once(&mut self.r)
                 .chain(once(&mut self.g))
                 .chain(once(&mut self.b)))
    }
}
