use std::str;
use std::process::Command;
use regex::Regex;


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
