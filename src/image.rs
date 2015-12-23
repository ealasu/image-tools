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


pub trait Pixel: Copy + Clone {
    fn magick_map() -> &'static str;
    //fn each_component<F>(&self, f: F) where F: Fn(f32);
    //fn write_as_u16<W: Writer>(w: W) -> IoResult<()>;
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct GrayPixel<T: Copy + Clone + Debug>(pub T);

impl<T> Pixel for GrayPixel<T> {
    fn magick_map() -> &'static str { "i" }

    //fn each_component<F>(&self, f: F) where F: Fn(T) {
        //f(self.0);
    //}

    //fn write_as_u16<W: Writer>(w: W) -> IoResult<()> {
        //w.write(self as 
    //}
}

impl From<GrayPixel<f32>> for GrayPixel<u16> {
    fn from(p: GrayPixel<f32>) -> Self {
        GrayPixel(p as u16)
    }
}


#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct RgbPixel<T: Copy + Clone + Debug> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> Pixel for RgbPixel<T> {
    fn magick_map() -> &'static str { "rgb" }

    //fn each_component<F>(&self, f: F) where F: Fn(f32) {
        //f(self.r);
        //f(self.g);
        //f(self.b);
    //}
}


#[derive(PartialEq)]
pub struct Image<P: Pixel> {
    pub width: usize,
    pub height: usize,
    data: Vec<P>,
}

impl<P: Pixel> fmt::Debug for Image<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[image {}x{}]", self.width, self.height)
    }
}

impl<P: Pixel> Image<P> {
    pub fn open(path: &str) -> Self {
        let out = Command::new("stream")
            .arg("-map")
            .arg(P::magick_map())
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
        Self::from_data(width, height, data)
    }

    pub fn save(&self, path: &str) {
        // convert to u16
        let shorts: Vec<u16> = Wrap(self.data.clone()).into();
        // then coerce to u8
        let data = Wrap(shorts).into();

        //let data = vec_of_f32_to_u8(self.data.clone());
        //let mut f = File::create(path).unwrap();
        //f.write_all(&data).unwrap();
        //return;

        // convert -size 5184x3456 -depth 16 gray:data/a.dat  data/x.tiff
        let mut child = Command::new("convert")
            .arg("-size").arg(format!("{}x{}", self.width, self.height))
            .arg("-depth").arg("16")
            //.arg("-define").arg("quantum:format=floating-point")
            .arg("gray:-")
            //.arg("FITS:-")
            .arg(path)
            .stdin(Stdio::piped())
            .spawn().unwrap();
        child.stdin.unwrap().write_all(&data).unwrap();
        //child.stdin.unwrap().flush().unwrap();
        //child.wait().unwrap();
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

    pub fn new(width: usize, height: usize) -> Self {
        let mut data = Vec::with_capacity(width * height);
        data.extend(repeat(0.0).take(width * height));
        Self::from_data(width, height, data)
    }

    pub fn from_data(width: usize, height: usize, data: Vec<P>) -> Self {
        Image {
            width: width,
            height: height,
            data: data,
        }
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

