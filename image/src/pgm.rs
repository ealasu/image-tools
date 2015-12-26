use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result as IoResult;
use std::fs::File;
use convert::*;


fn read_until<R: BufRead>(r: &mut R, until: char) -> IoResult<String> {
    let mut out = Vec::new();
    try!(r.read_until(until as u8, &mut out));
    let s = String::from_utf8(out).unwrap();
    Ok(s)
}

pub enum Format {
    U16(Vec<u16>),
    F32(Vec<f32>),
}

const U16_MAX: &'static str = "65536";
const U32_MAX: &'static str = "4294967296";

pub fn read<R: BufRead>(r: &mut R) -> IoResult<(usize, usize, Format)> {
    let magic = try!(read_until(r, '\n'));
    assert_eq!(magic, "P5\n");

    let w = try!(read_until(r, ' '));
    let h = try!(read_until(r, '\n'));
    let max_val = try!(read_until(r, '\n'));

    let w = w.trim().parse::<usize>().unwrap();
    let h = h.trim().parse::<usize>().unwrap();

    let mut data = Vec::new();
    try!(r.read_to_end(&mut data));

    let res = match max_val.trim() {
        U16_MAX => Format::U16(convert_vec(data)),
        U32_MAX => Format::F32(convert_vec(data)),
        _ => panic!("unsupported format: {}", max_val)
    };
    Ok((w, h, res))
}

pub fn read_from_file(path: &str) -> (usize, usize, Format) {
    let f = File::open(path).unwrap();
    let mut r = BufReader::new(f);
    read(&mut r).unwrap()
}

pub fn write<W: Write>(w: &mut W, width: usize, height: usize, data: Format) -> IoResult<()> {
    let (max_val, data): (_, Vec<u8>) = match data {
        Format::U16(data) => (U16_MAX, convert_vec(data)),
        Format::F32(data) => (U32_MAX, convert_vec(data)),
    };
    try!(w.write(format!("P5\n{} {}\n{}\n", width, height, max_val).as_bytes()));
    w.write_all(&data)
}

pub fn write_to_file(path: &str, w: usize, h: usize, data: Format) {
    let mut f = File::create(path).unwrap();
    write(&mut f, w, h, data).unwrap();
}
