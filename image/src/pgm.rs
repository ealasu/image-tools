use std::u16;
use std::io::prelude::*;
use std::io::Result as IoResult;
use convert::*;


fn read_until<R: BufRead>(r: &mut R, until: char) -> IoResult<String> {
    let mut out = Vec::new();
    try!(r.read_until(until as u8, &mut out));
    let s = String::from_utf8(out).unwrap();
    Ok(s)
}

pub fn read<R: BufRead>(r: &mut R) -> IoResult<(usize, usize, Vec<u16>)> {
    let magic = try!(read_until(r, '\n'));
    assert_eq!(magic, "P5\n");

    let w = try!(read_until(r, ' '));
    let h = try!(read_until(r, '\n'));
    let max_val = try!(read_until(r, '\n'));
    assert_eq!(max_val, format!("{}\n", u16::MAX));

    let w = w.trim().parse::<usize>().unwrap();
    let h = h.trim().parse::<usize>().unwrap();

    let mut data = Vec::new();
    try!(r.read_to_end(&mut data));
    let data = convert_vec(data);

    Ok((w, h, data))
}
