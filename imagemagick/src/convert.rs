use std::f32;
use std::u16;
use std::str;
use std::process::Command;
use std::process::Stdio;
use std::io::prelude::*;
use std::path::Path;
use regex::Regex;

pub fn convert_vec<From,To>(mut data: Vec<From>) -> Vec<To> {
    use std::mem;
    data.shrink_to_fit();
    let p = data.as_mut_ptr();
    let len = data.len() * mem::size_of::<From>() / mem::size_of::<To>();
    unsafe {
        mem::forget(data);
        Vec::from_raw_parts(p as *mut To, len, len)
    }
}

fn stretch(data: &[f32]) -> Vec<u16> {
    let src_min = data.iter().fold(f32::MAX, |acc, &v| acc.min(v));
    let src_max = data.iter().fold(f32::MIN, |acc, &v| acc.max(v));
    let src_d = src_max - src_min;
    let dst_min = u16::MIN as f32;
    let dst_max = u16::MAX as f32;
    let dst_d = dst_max - dst_min;
    data.iter().map(|&v| {
        (((v - src_min) * dst_d) / src_d) as u16
    }).collect()
}

pub fn convert_open<P: AsRef<Path>>(path: P, map: &str) -> (usize, usize, Vec<f32>) {
    let out = Command::new("convert")
        .arg("-verbose")
        .arg(path.as_ref())
        .arg("-depth").arg("32")
        .arg("-define").arg("quantum:format=floating-point")
        .arg(format!("{}:-", map))
        .output()
        .unwrap();
    assert!(out.status.success());
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

pub fn convert_save<P: AsRef<Path>>(data: &[f32], width: usize, height: usize, format: &str, magick_type: &str, path: P) {
    let data = stretch(data);
    let data: Vec<u8> = convert_vec(data);
    let child = Command::new("convert")
        .arg("-size").arg(format!("{}x{}", width, height))
        .arg("-depth").arg("16")
        //.arg("-define").arg("quantum:format=floating-point")
        .arg(format!("{}:-", format))
        //.arg("-depth").arg("16")
        .arg("-type").arg(magick_type)
        .arg(path.as_ref())
        .stdin(Stdio::piped())
        .spawn().unwrap();
    child.stdin.unwrap().write_all(&data).unwrap();
}
