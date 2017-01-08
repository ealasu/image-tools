use std::io::prelude::*;
use std::str;
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};

const RECORDS_PER_BLOCK: usize = 36;
const RECORD_LEN: usize = 80;
const NAME_LEN: usize = 8;
const TEXT_LEN: usize = RECORD_LEN - NAME_LEN - 2;

#[derive(Debug)]
pub struct HeaderRecord {
    pub name: String,
    pub value: Option<String>,
    pub comment: String,
}

pub fn read_header<R: Read>(r: &mut R) -> Vec<HeaderRecord> {
    let mut records = vec![];
    let mut records_read = 0;
    loop {
        let mut buf = [0u8; RECORD_LEN];
        r.read_exact(&mut buf).unwrap();
        records_read  += 1;
        let name = str::from_utf8(&buf[..NAME_LEN]).unwrap().trim().to_string();
        if name == "" {
            continue;
        }
        if name == "END" {
            for _ in 0..RECORDS_PER_BLOCK - records_read % RECORDS_PER_BLOCK {
                let mut buf = [0u8; RECORD_LEN];
                r.read_exact(&mut buf).unwrap();
            }
            break;
        }
        let text = str::from_utf8(&buf[10..]).unwrap().trim();
        let (value, comment) = if buf[8] == '=' as u8 && buf[9] == ' ' as u8 {
            let mut i = text.splitn(2, '/'); // TODO: handle quoted value
            let value = i.next().unwrap().trim().to_string();
            let comment = if let Some(s) = i.next() {
                s.trim().to_string()
            } else {
                "".to_string()
            };
            (Some(value), comment)
        } else {
            (None, text.trim().to_string())
        };
        records.push(HeaderRecord {
            name: name,
            value: value,
            comment: comment,
        });
    }
    records
}

pub fn write_header<W: Write>(w: &mut W, records: &[HeaderRecord]) {
    for record in records.iter() {
        // write the name
        let name = record.name.as_bytes();
        assert!(name.len() <= NAME_LEN);
        w.write_all(name).unwrap();
        write_padding(w, NAME_LEN - name.len());

        // write the value
        let comment = record.comment.as_bytes();
        if let Some(ref value) = record.value {
            w.write_all(b"= ").unwrap();
            let value = value.as_bytes();
            assert!(value.len() <= TEXT_LEN);
            w.write_all(value).unwrap();
            let mut text_written = value.len();
            if !comment.is_empty() {
                w.write_all(b"/").unwrap();
                assert!(comment.len() + 1 <= TEXT_LEN - value.len());
                w.write_all(comment).unwrap();
                text_written += 1 + comment.len();
            }
            write_padding(w, TEXT_LEN - text_written);
        } else {
            w.write_all(b"  ").unwrap();
            assert!(comment.len() <= TEXT_LEN);
            w.write_all(comment).unwrap();
            write_padding(w, TEXT_LEN - comment.len());
        }

    }

    w.write_all(b"END").unwrap();
    write_padding(w, RECORD_LEN - b"END".len());

    for _ in 0..(RECORDS_PER_BLOCK - (records.len() + 1) % RECORDS_PER_BLOCK) {
        let blanks = [' ' as u8; RECORD_LEN];
        w.write_all(&blanks[..]).unwrap();
    }
}

fn write_padding<W: Write>(w: &mut W, len: usize) {
    let padding = [b' '];
    for _ in 0..len {
        w.write_all(&padding[..]).unwrap();
    }
}

pub enum Data {
    U16(Vec<u16>),
    F32(Vec<f32>),
}

fn get_value<'a>(records: &'a [HeaderRecord], name: &str) -> &'a String {
    if let Some(ref r) = records
        .iter()
        .find(|r| r.name == name) {
        if let Some(ref v) = r.value {
            v
        } else {
            panic!("record with name {} does not have a value", name);
        }
    } else {
        panic!("could not find record with name {}", name);
    }
}

pub fn write_image<W: Write>(w: &mut W, width: usize, height: usize, data: &Data) {
    let bitpix = match data {
        &Data::U16(_) => "16",
        &Data::F32(_) => "-32",
    }.to_string();
    let header = [
        HeaderRecord {
            name: "SIMPLE".to_string(),
            value: Some("T".to_string()),
            comment: "".to_string(),
        },
        HeaderRecord {
            name: "BITPIX".to_string(),
            value: Some(bitpix),
            comment: "".to_string(),
        },
        HeaderRecord {
            name: "NAXIS".to_string(),
            value: Some("2".to_string()),
            comment: "".to_string(),
        },
        HeaderRecord {
            name: "NAXIS1".to_string(),
            value: Some(format!("{}", width)),
            comment: "".to_string(),
        },
        HeaderRecord {
            name: "NAXIS2".to_string(),
            value: Some(format!("{}", height)),
            comment: "".to_string(),
        },
        HeaderRecord {
            name: "NAXIS2".to_string(),
            value: Some(format!("{}", height)),
            comment: "".to_string(),
        },
    ];
    write_header(w, &header[..]);
    match data {
        &Data::U16(ref vec) => {
            for &v in vec.iter() {
                w.write_u16::<BigEndian>(v).unwrap();
            }
        },
        &Data::F32(ref vec) => {
            for &v in vec.iter() {
                w.write_f32::<BigEndian>(v).unwrap();
            }
        },
    }
}

pub fn read_image<R: Read>(r: &mut R) -> (usize, usize, Data) {
    let records = read_header(r);
    let naxis = get_value(&records, "NAXIS");
    assert_eq!(naxis, "2");
    let width = get_value(&records, "NAXIS1").parse::<usize>().unwrap();
    let height = get_value(&records, "NAXIS2").parse::<usize>().unwrap();
    let bitpix = get_value(&records, "BITPIX");
    let data_len = width * height;
    let data = match bitpix.as_str() {
        "16" => {
            let mut data = vec![];
            for _ in 0..data_len {
                data.push(r.read_u16::<BigEndian>().unwrap());
            }
            Data::U16(data)
        },
        "-32" => {
            let mut data = vec![];
            for _ in 0..data_len {
                data.push(r.read_f32::<BigEndian>().unwrap());
            }
            Data::F32(data)
        },
        _ => panic!("unexpected BITPIX: {}", bitpix)
    };
    (width, height, data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_read() {
        let mut f = File::open("test/a.fits").unwrap();
        let h = read_header(&mut f);
        for v in h.iter() {
            println!("{:?}", v);
        }
        //let (w,h,d) = read_image(&mut f);
        //println!("{}x{}", w, h);
    }
}