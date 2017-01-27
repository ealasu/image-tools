extern crate regex;
extern crate tempdir;

use std::process::Command;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;
use tempdir::TempDir;

#[derive(Debug)]
pub struct Object {
    flux: f32,
    x: f32,
    y: f32,
}

pub fn extract(path: &str) -> Vec<Object> {
    let temp_dir = TempDir::new("sextractor").expect("create temp dir");

    {
        let mut f = File::create(temp_dir.path().join("default.sex")).unwrap();
        f.write_all(include_str!("config/default.sex").as_bytes()).unwrap();
    }
    {
        let mut f = File::create(temp_dir.path().join("default.param")).unwrap();
        f.write_all(include_str!("config/default.param").as_bytes()).unwrap();
    }

    let mut status = Command::new("sex")
        .current_dir(temp_dir.path())
        .arg(fs::canonicalize(path).unwrap())
        .status()
        .expect("failed to execute sex");
    assert!(status.success());

    let mut r = BufReader::new(File::open(temp_dir.path().join("test.cat")).unwrap());
    let whitespace = Regex::new(r"\s+").unwrap();
    r.lines().map(|line| {
        let line = line.unwrap();
        let mut cols = whitespace.split(line.trim());
        let flux = cols.next().unwrap();
        let x = cols.next().unwrap();
        let y = cols.next().unwrap();
        Object {
            flux: flux.parse().unwrap(),
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let res = extract("test/a.fits");
        println!("len: {}", res.len());
        println!("{:?}", res[0]);
    }
}
