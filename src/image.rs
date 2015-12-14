use std::str;
use std::mem;
use std::fmt;
use std::process::Command;
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
    pub fn load(path: &str) -> Image {
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
        Self::new(width, height, vec_of_u8_to_f32(out.stdout))
    }

    pub fn new(width: usize, height: usize, data: Vec<Pixel>) -> Image {
        Image {
            width: width,
            height: height,
            data: data,
        }
    }

    #[inline(always)]
    pub fn at(&self, x: usize, y: usize) -> Pixel {
        self.data[x + y * self.width]
    }

    #[inline(always)]
    pub fn pixels(&self) -> &Vec<Pixel> {
        &self.data
    }

    pub fn row(&self, y: usize, left: usize, right: usize) -> &[Pixel] {
        let start = y * self.width;
        &self.data[start + left .. start + right]
    }

    pub fn crop(&self, left: usize, top: usize, right: usize, bottom: usize) -> Image {
        assert!(right > left);
        assert!(bottom > top);
        let width = right - left;
        let height = bottom - top;
        let mut data = Vec::with_capacity(width * height);
        for y in top..bottom {
            data.extend_from_slice(self.row(y, left, right));
        }
        Self::new(width, height, data)
    }
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

