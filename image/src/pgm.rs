use std::io::prelude::*;
use std::io::Result as IoResult;
//use convert::*;
use byteorder::{ReadBytesExt, BigEndian as BE};

fn read_until<R: BufRead>(r: &mut R, until: char) -> IoResult<String> {
    let mut out = Vec::new();
    try!(r.read_until(until as u8, &mut out));
    let s = String::from_utf8(out).unwrap();
    Ok(s)
}

const U16_MAX: &'static str = "65535";

pub fn read<R: BufRead>(r: &mut R) -> IoResult<(usize, usize, Vec<u16>)> {
    let magic = read_until(r, '\n')?;
    assert_eq!(magic, "P5\n");

    let w = read_until(r, ' ')?;
    let h = read_until(r, '\n')?;
    let max_val = read_until(r, '\n')?;

    let w = w.trim().parse::<usize>().unwrap();
    let h = h.trim().parse::<usize>().unwrap();
    assert_eq!(max_val.trim(), U16_MAX);

    let len = w * h;
    let mut pixels = Vec::with_capacity(len);
    for _ in 0..len {
        let v = r.read_u16::<BE>()?;
        pixels.push(v);
    }
    Ok((w, h, pixels))
}

//pub fn read_from_file(path: &str) -> (usize, usize, Format) {
    //let f = File::open(path).unwrap();
    //let mut r = BufReader::new(f);
    //read(&mut r).unwrap()
//}

//pub fn write<W: Write>(w: &mut W, width: usize, height: usize, data: Format) -> IoResult<()> {
    //let (max_val, data): (_, Vec<u8>) = match data {
        //Format::U16(data) => (U16_MAX, convert_vec(data)),
        //Format::F32(data) => (U32_MAX, convert_vec(data)),
    //};
    //try!(w.write(format!("P5\n{} {}\n{}\n", width, height, max_val).as_bytes()));
    //w.write_all(&data)
//}

//pub fn write_to_file(path: &str, w: usize, h: usize, data: Format) {
    //let mut f = File::create(path).unwrap();
    //write(&mut f, w, h, data).unwrap();
//}
