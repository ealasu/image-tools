use tempfile::NamedTempFile;
use std::process::Command;
use std::fs;
use std::path::Path;

pub struct Camera {
    pub keep_raw: bool,
}

impl Camera {
    pub fn new() -> Self {
        let status = Command::new("umount")
            .arg("/mnt/ramdisk")
            .status()
            .expect("failed to execute umount");
        assert!(status.success());
        let status = Command::new("mount")
            .arg("-a")
            .status()
            .expect("failed to execute mount");
        assert!(status.success());

        Camera {
            keep_raw: true,
        }
    }

    pub fn shoot(&self) -> NamedTempFile {
        let tmpdir = Path::new("/mnt/ramdisk");

        let jpeg_file = NamedTempFile::new_in(tmpdir).unwrap();
        debug!("saving jpeg to {:?}", jpeg_file.path());
        let mut command = Command::new("gphoto2");
        command
            .arg("--filename").arg(jpeg_file.path())
            .arg("--force-overwrite");
        if self.keep_raw {
            // Choice: 1 Memory card
            command
                .arg("--set-config").arg("capturetarget=1")
                .arg("--keep-raw")
                // Choice: 8 RAW + Large Fine JPEG
                .arg("--set-config").arg("imageformat=8");
        } else {
            command
                // Choice: 0 Large Fine JPEG
                .arg("--set-config").arg("imageformat=0");
        }
        command
            // Choice: 37 1/160
            //.arg("--set-config").arg("shutterspeed=37")
            // Choice: 1 100
            //.arg("--set-config").arg("iso=1")

            // 20
            .arg("--set-config").arg("shutterspeed=3")
            // 5000
            .arg("--set-config").arg("iso=18")
            .arg("--capture-image-and-download");

        let status = command
            .status()
            .expect("failed to execute gphoto2");
        assert!(status.success());
        debug!("jpeg file len: {}", fs::metadata(jpeg_file.path()).unwrap().len());
        fs::copy(jpeg_file.path(), tmpdir.join(Path::new("latest.jpeg"))).unwrap();

        // convert IMG_3332.JPG -gravity center -extent 900x900 -crop 900x900^ -auto-level out.jpeg
        let fits_file = NamedTempFile::new_in(tmpdir).unwrap();
        debug!("saving fits to {:?}", fits_file.path());
        let status = Command::new("convert")
            .arg(format!("jpeg:{}", jpeg_file.path().to_str().unwrap()))
            .arg("-gravity").arg("center")
            //.arg("-extent").arg("900x900")
            //.arg("-crop").arg("900x900")
            .arg("-extent").arg("2000x2000")
            .arg("-crop").arg("2000x2000")
            .arg("-depth").arg("16")
            .arg("-set").arg("colorspace").arg("Gray")
            .arg("-separate")
            .arg("-average")
            .arg(format!("fits:{}", fits_file.path().to_str().unwrap()))
            .status()
            .expect("failed to execute convert");
        assert!(status.success());
        debug!("fits file len: {}", fs::metadata(fits_file.path()).unwrap().len());

        fits_file
    }
}
