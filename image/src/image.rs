use std::str;
use std::default::Default;
use std::fmt;
use std::f32;
use std::u16;
use std::process::Command;
use std::iter::repeat;
use std::process::Stdio;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::once;
use regex::Regex;
use convert::*;
use pgm;


pub struct Channel<P> {
    pub width: usize,
    pub height: usize,
    data: Vec<P>,
}

impl<P> fmt::Debug for Channel<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[image {}x{}]", self.width, self.height)
    }
}

impl<P: Clone + Copy + Default> Channel<P> {
    pub fn from_data(width: usize, height: usize, data: Vec<P>) -> Self {
        Channel {
            width: width,
            height: height,
            data: data,
        }
    }

    pub fn new(width: usize, height: usize) -> Self {
        let mut data: Vec<P> = Vec::with_capacity(width * height);
        let zero: P = Default::default();
        data.extend(repeat(zero).take(width * height));
        Self::from_data(width, height, data)
    }

    #[inline(always)]
    pub fn at(&self, x: usize, y: usize) -> P {
        //assert!(x < self.width);
        //assert!(y < self.height);
        self.data[x + y * self.width]
    }

    #[inline(always)]
    pub fn at_mut(&mut self, x: usize, y: usize) -> &mut P {
        //assert!(x < self.width);
        //assert!(y < self.height);
        &mut self.data[x + y * self.width]
    }

    #[inline(always)]
    pub fn pixels(&self) -> &Vec<P> {
        &self.data
    }

    #[inline(always)]
    pub fn pixels_mut(&mut self) -> &mut Vec<P> {
        &mut self.data
    }
}

//impl Channel<f32> {
    //fn rescale_to_u16(&self) -> Channel<u16> {
        //let data = self.pixels();
        //let src_min = data.iter().fold(f32::MAX, |acc, &v| acc.min(v));
        //let src_max = data.iter().fold(f32::MIN, |acc, &v| acc.max(v));
        //let src_d = src_max - src_min;
        //let dst_min = u16::MIN as f32;
        //let dst_max = u16::MAX as f32;
        //let dst_d = dst_max - dst_min;

        //let mut out: Vec<u16> = Vec::with_capacity(data.len());
        //for v in data.iter() {
            //out.push((((*v - src_min) * dst_d) / src_d) as u16);
        //}
        //Channel::from_data(self.width, self.height, out)
    //}
//}


fn rescale(data: &[f32]) -> Vec<u16> {
    let src_min = data.iter().fold(f32::MAX, |acc, &v| acc.min(v));
    let src_max = data.iter().fold(f32::MIN, |acc, &v| acc.max(v));
    let src_d = src_max - src_min;
    let dst_min = u16::MIN as f32;
    let dst_max = u16::MAX as f32;
    let dst_d = dst_max - dst_min;

    let mut out: Vec<u16> = Vec::with_capacity(data.len());
    for v in data.iter() {
        out.push((((*v - src_min) * dst_d) / src_d) as u16);
    }
    out
}

fn magick_stream(path: &str, map: &str) -> (usize, usize, Vec<f32>) {
    let out = Command::new("convert")
        .arg("-verbose")
        .arg(path)
        .arg("-depth").arg("32")
        .arg("-define").arg("quantum:format=floating-point")
        .arg(format!("{}:-", map))
        .output()
        .unwrap();
    let stderr = str::from_utf8(&out.stderr).unwrap();
    println!("stderr: {}", stderr);
    let re = Regex::new(r" (\d+)x(\d+) ").unwrap();
    let captures = re.captures(stderr).unwrap();
    let width = captures[1].parse().unwrap();
    let height = captures[2].parse().unwrap();
    println!("u8 data len: {}", out.stdout.len());
    let data = convert_vec(out.stdout);
    (width, height, data)
}

fn magick_convert(data: &[f32], width: usize, height: usize, format: &str, magick_type: &str, path: &str) {
    let data = rescale(data);
    let data: Vec<u8> = convert_vec(data);
    let child = Command::new("convert")
        .arg("-size").arg(format!("{}x{}", width, height))
        .arg("-depth").arg("16")
        //.arg("-define").arg("quantum:format=floating-point")
        .arg(format!("{}:-", format))
        //.arg("-depth").arg("16")
        .arg("-type").arg(magick_type)
        .arg(path)
        .stdin(Stdio::piped())
        .spawn().unwrap();
    child.stdin.unwrap().write_all(&data).unwrap();
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct Rgb {
    r: f32,
    g: f32,
    b: f32,
}


pub trait Image<P> {
    fn new(width: usize, height: usize) -> Self;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn channels<'a>(&'a self) -> Box<Iterator<Item=&'a Channel<P>> + 'a>;
    fn channels_mut<'a>(&'a mut self) -> Box<Iterator<Item=&'a mut Channel<P>> + 'a>;
}

//pub trait RescaleTo<From,To>: Image<From> {
    //fn rescale_to<I: Image<To>>(&self) -> I;
//}

//impl RescaleTo<u16, f32> for Image<u16> {
    //fn rescale_to<I: Image<u32>>(&self) -> I;
//}


#[derive(Debug)]
pub struct GrayImage<P>(pub Channel<P>);

impl GrayImage<f32> {
    pub fn open(path: &str) -> Self {
        let (width, height, data) = magick_stream(path, "gray");
        GrayImage(Channel::from_data(width, height, data))
    }

    pub fn save(&self, path: &str) {
        magick_convert(self.0.pixels(), self.width(), self.height(), "gray", "grayscale", path);
    }
}

impl GrayImage<u16> {
    pub fn open_raw(path: &str) -> Self {
        let out = Command::new("dcraw")
            .arg("-c") // to stdout
            .arg("-4")
            .arg("-D")
            .arg(path)
            .output()
            .unwrap();
        let stderr = str::from_utf8(&out.stderr).unwrap();
        println!("stderr: {}", stderr);
        let mut r = BufReader::new(&out.stdout[..]);
        let (w, h, data) = pgm::read(&mut r).unwrap();
        GrayImage(Channel::from_data(w, h, data))
    }

    fn rescale_to_f32(&self) -> GrayImage<f32> {
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


pub fn identify(path: &str) -> (usize, usize) {
    let out = Command::new("identify")
        .arg(path)
        .output()
        .unwrap();
    let stdout = str::from_utf8(&out.stdout).unwrap();
    let re = Regex::new(r" (\d+)x(\d+) ").unwrap();
    let captures = re.captures(stdout).unwrap();
    let width = captures[1].parse().unwrap();
    let height = captures[2].parse().unwrap();
    (width, height)
}