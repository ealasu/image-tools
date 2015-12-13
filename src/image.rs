use std::str;
use std::mem;
use std::process::Command;
use regex::Regex;


pub struct Image {
    pub width: usize,
    pub height: usize,
    data: Vec<u16>
}

impl Image {
    pub fn load(path: &str) -> Image {
        let out = Command::new("stream")
            .arg("-map")
            .arg("i")
            .arg("-storage-type")
            .arg("short")
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

        Image {
            width: width,
            height: height,
            data: vec_of_u8_to_u16(out.stdout),
        }
    }

    pub fn at(&self, x: usize, y: usize) -> u16 {
        self.data[x + y * self.width]
    }

    pub fn pixels(&self) -> &Vec<u16> {
        &self.data
    }
}

fn vec_of_u8_to_u16(mut data: Vec<u8>) -> Vec<u16> {
    data.shrink_to_fit();
    let p = data.as_mut_ptr();
    let len = data.len() / 2;
    unsafe {
        mem::forget(data);
        Vec::from_raw_parts(p as *mut u16, len, len)
    }
}

