use tempfile::NamedTempFile;
use std::process::Command;
use std::fs;
use std::path::Path;

pub fn shoot() -> NamedTempFile {
    let keep_raw = false;

    let tmpdir = Path::new("/mnt/ramdisk");

    let jpeg_file = NamedTempFile::new_in(tmpdir).unwrap();
    let mut command = Command::new("gphoto2");
    command
        .arg("--filename").arg(jpeg_file.path())
        .arg("--force-overwrite");
    if keep_raw {
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
        // 5
        .arg("--set-config").arg("shutterspeed=5")

        // 6400
        .arg("--set-config").arg("iso=19")

        .arg("--capture-image-and-download");

    let status = command
        .status()
        .expect("failed to execute gphoto2");
    assert!(status.success());

    jpeg_file
}
