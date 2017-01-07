use std::io::prelude::*;
use std::str;

const RECORD_COUNT: usize = 36;
const RECORD_LEN: usize = 80;
const NAME_LEN: usize = 8;
const TEXT_LEN: usize = RECORD_LEN - NAME_LEN - 2;

pub struct HeaderRecord {
    name: String,
    value: Option<String>,
    comment: String,
}

pub fn read_header<R: Read>(r: &mut R) -> Vec<HeaderRecord> {
    let mut records = vec![];
    for _ in 0..RECORD_COUNT {
        let mut buf = [0u8; RECORD_LEN];
        r.read_exact(&mut buf).unwrap();
        let name = str::from_utf8(&buf[..NAME_LEN]).unwrap().to_string();
        if name.trim() == "" {
            continue;
        }
        let text = str::from_utf8(&buf[10..]).unwrap();
        let (value, comment) = if buf[8] == '=' as u8 && buf[9] == ' ' as u8 {
            let mut i = text.splitn(2, '/');
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
    assert!(records.len() <= RECORD_COUNT);
    for record in records.iter() {
        // write the name
        let name = record.name.as_bytes();
        assert!(name.len() <= NAME_LEN);
        w.write_all(name).unwrap();
        let padding = [b' '];
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
    for _ in 0..(RECORD_COUNT - records.len()) {
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
