use std::str;
use std::process::Command;
use std::io::BufReader;
use pgm;


pub fn open_raw(path: &str) -> (usize, usize, Vec<u16>) {
    let out = Command::new("dcraw")
        .arg("-c") // to stdout
        .arg("-4")
        .arg("-D")
        .arg(path)
        .output()
        .unwrap();
    let stderr = str::from_utf8(&out.stderr).unwrap();
    println!("stderr: {}", stderr);
    let mut r = BufReader::new(&out.stdout[..]);
    let (w, h, pixels) = pgm::read(&mut r).unwrap();
    (w, h, pixels)
}
