extern crate fits;
extern crate regex;
extern crate tempfile;

use tempfile::NamedTempFile;
use std::process::Command;
use std::fs;
use std::fs::File;
use std::path::Path;
use regex::Regex;

pub fn solve(path: &str) -> (f64, f64) {
    //let wcs_file = NamedTempFile::new().unwrap();

    let mut output = Command::new("solve-field")
        .arg("--scale-units").arg("degwidth")
        .arg("--scale-low").arg("10.2")
        .arg("--scale-high").arg("10.4")
        .arg("--overwrite")
        .arg("--downsample").arg("4")
        .arg("--no-plots")
        .arg("--new-fits").arg("none")
        .arg("--wcs").arg("none")
        .arg("--match").arg("none")
        .arg("--rdls").arg("none")
        .arg("--solved").arg("none")
        .arg("--corr").arg("none")
        .arg("--index-xyls").arg("none")
        .arg("--temp-axy")
        .arg("--parity").arg("neg")
        .arg(path)
        .output()
        .expect("failed to execute solve-field");
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("stdout: {}", stdout);
    assert!(output.status.success());

    let cap = Regex::new(r"Field center: \(RA,Dec\) = \((.+), (.+)\) deg").unwrap().captures_iter(&stdout).next().unwrap();
    let ra = cap[1].parse::<f64>().unwrap();
    let dec = cap[2].parse::<f64>().unwrap();
    println!("ra: {} dec: {}", ra, dec);

    (ra, dec)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use fits;

    #[test]
    fn test_s() {
        solve("/mnt/ramdisk/capt0000.jpg");
    }

    //#[test]
    fn test_read() {
        let mut f = File::open("/mnt/ramdisk/capt0000.wcs").unwrap();
        let h = fits::read_header(&mut f);
        for v in h.iter() {
            println!("{:?}", v);
        }
        //let (w,h,d) = read_image(&mut f);
        //println!("{}x{}", w, h);
    }
}
