use std::str;
use std::fmt;
use std::f32;
use std::u16;
use std::process::Command;
use std::iter::repeat;
use std::process::Stdio;
use std::io::prelude::*;
use regex::Regex;
use convert::Wrap;


pub type Pixel = f32;

pub struct Channel {
    pub width: usize,
    pub height: usize,
    data: Vec<Pixel>,
}

impl fmt::Debug for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[image {}x{}]", self.width, self.height)
    }
}

impl Channel {
    pub fn from_data(width: usize, height: usize, data: Vec<Pixel>) -> Self {
        Channel {
            width: width,
            height: height,
            data: data,
        }
    }

    pub fn new(width: usize, height: usize) -> Self {
        let mut data = Vec::with_capacity(width * height);
        data.extend(repeat(0.0).take(width * height));
        Self::from_data(width, height, data)
    }

    #[inline(always)]
    pub fn at(&self, x: usize, y: usize) -> Pixel {
        //assert!(x < self.width);
        //assert!(y < self.height);
        self.data[x + y * self.width]
    }

    #[inline(always)]
    pub fn at_mut(&mut self, x: usize, y: usize) -> &mut Pixel {
        //assert!(x < self.width);
        //assert!(y < self.height);
        &mut self.data[x + y * self.width]
    }

    #[inline(always)]
    pub fn pixels(&self) -> &Vec<Pixel> {
        &self.data
    }

    #[inline(always)]
    pub fn pixels_mut(&mut self) -> &mut Vec<Pixel> {
        &mut self.data
    }
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
    let Wrap(data) = Wrap::from(out.stdout);
    (width, height, data)
}

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

fn magick_convert(data: &[f32], width: usize, height: usize, format: &str, magick_type: &str, path: &str) {
    let data = rescale(data);
    let Wrap(data): Wrap<Vec<u8>> = Wrap::from(data);
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
struct Rgb {
    r: f32,
    g: f32,
    b: f32,
}

impl_from_to!(Rgb, f32);
impl_from_to!(f32, Rgb);


#[derive(Debug)]
pub struct Image {
    pub channels: Vec<Channel>,
    pub width: usize,
    pub height: usize,
}

impl Image {
    pub fn open_gray(path: &str) -> Image {
        let (width, height, data) = magick_stream(path, "gray");
        let layer = Channel::from_data(width, height, data);
        Self::new(vec![layer])
    }

    pub fn open_rgb(path: &str) -> Image {
        let (width, height, data) = magick_stream(path, "rgb");
        let Wrap(data): Wrap<Vec<Rgb>> = Wrap::from(data);
        assert_eq!(data.len(), width * height);
        let r = data.iter().map(|&p| p.r).collect();
        let g = data.iter().map(|&p| p.r).collect();
        let b = data.iter().map(|&p| p.r).collect();
        Self::new(vec![
            Channel::from_data(width, height, r),
            Channel::from_data(width, height, g),
            Channel::from_data(width, height, b),
        ])
    }

    pub fn save_gray(&self, path: &str) {
        assert_eq!(self.channels.len(), 1);
        let layer = &self.channels[0];
        magick_convert(layer.pixels(), layer.width, layer.height, "gray", "grayscale", path);
    }

    pub fn save_rgb(&self, path: &str) {
        assert_eq!(self.channels.len(), 3);
        let r = &self.channels[0];
        let g = &self.channels[1];
        let b = &self.channels[2];
        let rgb = (0..r.width * r.height).map(|i| {
            Rgb {
                r: r.pixels()[i],
                g: g.pixels()[i],
                b: b.pixels()[i],
            }
        }).collect::<Vec<_>>();
        let Wrap(data) = Wrap::from(rgb);
        magick_convert(&data, r.width, r.height, "rgb", "truecolor", path);
    }

    pub fn new(channels: Vec<Channel>) -> Self {
        assert!(channels.len() > 0);
        let w = channels[0].width;
        let h = channels[0].height;
        for c in channels.iter() {
            assert_eq!(c.width, w);
            assert_eq!(c.height, h);
        }
        Image {
            width: w,
            height: h,
            channels: channels,
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
}
