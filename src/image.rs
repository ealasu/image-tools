use std::str;
use std::fmt;
use std::mem;
use std::process::Command;
use std::iter::repeat;
use std::process::Stdio;
use std::io::prelude::*;
use std::fs::File;
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
    let out = Command::new("stream")
        .arg("-map")
        .arg(map)
        .arg("-storage-type")
        .arg("float")
        .arg("-verbose")
        .arg(path)
        .arg("-")
        .output()
        .unwrap();
    let stderr = str::from_utf8(&out.stderr).unwrap();
    let re = Regex::new(r" (\d+)x(\d+) ").unwrap();
    let captures = re.captures(stderr).unwrap();
    let width = captures[1].parse().unwrap();
    let height = captures[2].parse().unwrap();
    let Wrap(data) = Wrap::from(out.stdout);
    (width, height, data)
}

fn magick_convert(data: &[f32], width: usize, height: usize, format: &str, path: &str) {
    let Wrap(data): Wrap<Vec<u8>> = Wrap::from(data.to_vec());
    let mut child = Command::new("convert")
        .arg("-size").arg(format!("{}x{}", width, height))
        .arg("-depth").arg("32")
        .arg("-define").arg("quantum:format=floating-point")
        .arg(format!("{}:-", format))
        //.arg("FITS:-")
        .arg("-depth").arg("16")
        .arg(path)
        .stdin(Stdio::piped())
        .spawn().unwrap();
    child.stdin.unwrap().write_all(&data).unwrap();
}


#[repr(C, packed)]
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
        let (width, height, data) = magick_stream(path, "i");
        let layer = Channel::from_data(width, height, data);
        Self::new(vec![layer])
    }

    pub fn open_rgb(path: &str) -> Image {
        let (width, height, data) = magick_stream(path, "rgb");
        let Wrap(data): Wrap<Vec<Rgb>> = Wrap::from(data);
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
        magick_convert(layer.pixels(), layer.width, layer.height, "gray", path);

        //let data = vec_of_f32_to_u8(self.data.clone());
        //let mut f = File::create(path).unwrap();
        //f.write_all(&data).unwrap();
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
        magick_convert(&data, r.width, r.height, "rgb", path);
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


    //pub fn iter_pixels(&mut self) -> () {
        //let mut i = (0..);
        //(0..self.height).map(|y| {
            //(0..self.width).map(|x| {
                //(x, y, self.data[i.next().unwrap()]);
            //})
        //})
    //}

    //pub fn row(&self, y: usize, left: usize, right: usize) -> &[Pixel] {
        //let start = y * self.width;
        //&self.data[start + left .. start + right]
    //}

    //pub fn crop(&self, left: usize, top: usize, right: usize, bottom: usize) -> Image {
        //assert!(right > left);
        //assert!(bottom > top);
        //let width = right - left;
        //let height = bottom - top;
        //let mut data = Vec::with_capacity(width * height);
        //for y in top..bottom {
            //data.extend_from_slice(self.row(y, left, right));
        //}
        //Self::new(width, height, data)
    //}
}

