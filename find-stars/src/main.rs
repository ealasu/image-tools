extern crate image;
extern crate geom;
extern crate byteorder;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::BTree;
use image::{Image, Rgb, ImageKind};
use geom::Point;
use byteorder::{ReadBytesExt, LittleEndian as LE};


fn find_stars(image: &Image<f32>) -> Vec<Point<usize>> {
    let (min, max) = image.min_max();
    println!("min: {} max: {}", min, max);
    let avg = image.average();
    println!("avg: {}", avg);
    let fg_threshold = avg + 0.3 * (max - avg);
    let bg_threshold = avg + 0.1 * (max - avg);

    let ring_radius = 7;
    let ring: Vec<(isize, isize)> =
        (-ring_radius..ring_radius).map(|x| (x, ring_radius)).chain(
        (-ring_radius..ring_radius).map(|x| (x, -ring_radius))).chain(
        (-ring_radius..ring_radius).map(|y| (ring_radius, y))).chain(
        (-ring_radius..ring_radius).map(|y| (-ring_radius, y)))
        .collect();

    let margin = ring_radius as usize;

    let mut res = Vec::new();
    for y in margin .. image.height - margin {
        for x in margin .. image.width - margin {
            let p = image.pixel_at(x, y);
            if p > &fg_threshold &&
                p > image.pixel_at(x-1, y) &&
                p > image.pixel_at(x-1, y+1) &&
                p > image.pixel_at(x, y+1) &&
                p > image.pixel_at(x+1, y+1) &&
                p > image.pixel_at(x+1, y) &&
                p > image.pixel_at(x+1, y-1) &&
                p > image.pixel_at(x, y-1) &&
                p > image.pixel_at(x-1, y-1) &&
                ring.iter().all(|&(r_x, r_y)| {
                    image.pixel_at((x as isize + r_x) as usize, (y as isize + r_y) as usize) < &bg_threshold
                })
            {
                res.push(Point { x: x, y: y });
            }
        }
    }
    res
}

pub struct Coord {
    pub ra: f32,
    pub dec: f32,
}

fn read_catalog(path: &str) -> Vec<Coord> {
    let mut f = BufReader::new(File::open(path).unwrap());
    let mut res = Vec::new();
    loop {
        let ra = match f.read_f32::<LE>() {
            Ok(v) => v,
            Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
            e => e.unwrap()
        };
        let dec = f.read_f32::<LE>().unwrap();
        res.push(Coord { ra: ra, dec: dec });
    }
    res
}

pub struct Index {
    tree: BTree<u16, BTree<u16, BTree<u16, BTree<u16, Coord>>>>,
}

impl Index {
    pub fn new(catalog: &[Coord]) -> Self {
    }

    pub fn lookup(&self) -> Option<Coord> {
    }
}


fn main() {
    let img = Image::<f32>::open("crop.jpg");
    let stars = find_stars(&img);
    println!("found {} stars", stars.len());
    let mut f = File::create("stars.txt").unwrap();
    for p in stars.iter() {
        writeln!(f, "{},{}", p.x, p.y).unwrap();
    }
    let catalog = read_catalog("tyc2.dat");
    println!("catalog has {} stars", catalog.len());
    build_index(&catalog);

}
