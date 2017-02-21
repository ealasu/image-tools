extern crate tempfile;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate log;

mod errors;

use tempfile::NamedTempFile;
use std::process::Command;
use std::path::Path;
use std::thread;
use std::time::Duration;
use std::fs;
pub use errors::*;

pub struct Options {
    pub keep_raw: bool,
    pub shutter_speed: String,
    pub iso: String,
}

pub fn shoot(options: Options) -> Result<NamedTempFile> {
    let res = Command::new("pkill")
        .arg("PTPCamera")
        .status()
        .expect("failed to execute pkill");

    if res.success() {
        thread::sleep(Duration::from_secs(1));
    }

    let tmpdir = Path::new("/mnt/ramdisk");

    let jpeg_file = NamedTempFile::new_in(tmpdir).unwrap();
    let mut command = Command::new("gphoto2");
    command
        .arg("--filename").arg(jpeg_file.path())
        .arg("--force-overwrite");
    if options.keep_raw {
        // Choice: 1 Memory card
        command
            .arg("--set-config").arg("capturetarget=1")
            .arg("--keep-raw")
            // Choice: 8 RAW + Large Fine JPEG
            .arg("--set-config").arg("imageformat=8");
    } else {
        command
            .arg("--set-config").arg("capturetarget=0")
            // Choice: 0 Large Fine JPEG
            .arg("--set-config").arg("imageformat=0");
    }
    command
        // option 6
        .arg("--set-config").arg(format!("shutterspeed={}", options.shutter_speed))

        // ISO 6400
        // option 20
        .arg("--set-config").arg(format!("iso={}", options.iso))

        .arg("--capture-image-and-download");

    let status = command
        .status()
        .expect("failed to execute gphoto2");
    if !status.success() {
        return Err(ErrorKind::GphotoCommandFailed.into());
    }
    debug!("jpeg file len: {}", fs::metadata(jpeg_file.path()).unwrap().len());
    if jpeg_file.metadata()?.len() == 0 {
        return Err(ErrorKind::EmptyFile.into());
    }
    fs::copy(jpeg_file.path(), tmpdir.join(Path::new("latest.jpg"))).unwrap();

    Ok(jpeg_file)
}
