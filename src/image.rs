use std::str;
use std::mem;
use std::fmt;
use std::process::Command;
use std::iter::repeat;
use std::process::Stdio;
use std::io::prelude::*;
use std::fs::File;
use std::f32;
use std::u16;
use regex::Regex;


pub type Pixel = f32;

#[derive(PartialEq)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    data: Vec<Pixel>,
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[image {}x{}]", self.width, self.height)
    }
}

impl Image {
    pub fn open(path: &str) -> Image {
        let out = Command::new("stream")
            .arg("-map")
            .arg("i")
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
        Self::from_data(width, height, vec_of_u8_to_f32(out.stdout))
    }

    pub fn save(&self, path: &str) {
        let shorts = vec_of_f32_to_u16(&self.data);
        let data = vec_of_u16_to_u8(shorts);

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

    pub fn new(width: usize, height: usize) -> Image {
        let mut data = Vec::with_capacity(width * height);
        data.extend(repeat(0.0).take(width * height));
        Self::from_data(width, height, data)
    }

    pub fn from_data(width: usize, height: usize, data: Vec<Pixel>) -> Image {
        Image {
            width: width,
            height: height,
            data: data,
        }
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

fn vec_of_u8_to_f32(mut data: Vec<u8>) -> Vec<f32> {
    data.shrink_to_fit();
    let p = data.as_mut_ptr();
    let len = data.len() / 4;
    unsafe {
        mem::forget(data);
        Vec::from_raw_parts(p as *mut f32, len, len)
    }
}

fn vec_of_u16_to_u8(mut data: Vec<u16>) -> Vec<u8> {
    data.shrink_to_fit();
    let p = data.as_mut_ptr();
    let len = data.len() * 2;
    unsafe {
        mem::forget(data);
        Vec::from_raw_parts(p as *mut u8, len, len)
    }
}

fn vec_of_f32_to_u16(data: &[f32]) -> Vec<u16> {
    // rescale
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

fn vec_of_f32_to_u8(mut data: Vec<f32>) -> Vec<u8> {
    data.shrink_to_fit();
    let p = data.as_mut_ptr();
    let len = data.len() * 4;
    unsafe {
        mem::forget(data);
        Vec::from_raw_parts(p as *mut u8, len, len)
    }
}
