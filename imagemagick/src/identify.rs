use std::str;
use std::path::Path;
use std::process::Command;
use regex::Regex;

pub struct ImageInfo {
    pub width: usize,
    pub height: usize,
}

pub fn identify<P: AsRef<Path>>(path: P) -> ImageInfo {
    let out = Command::new("identify")
        .arg(path.as_ref())
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = str::from_utf8(&out.stdout).unwrap();
    let re = Regex::new(r" (\d+)x(\d+) ").unwrap();
    let captures = re.captures(stdout).unwrap();
    let width = captures[1].parse().unwrap();
    let height = captures[2].parse().unwrap();
    ImageInfo {
        width: width,
        height: height,
    }
}
