use tempfile::NamedTempFile;
use std::process::Command;

pub struct Camera {
}

impl Camera {
    pub fn new() -> Self {
        Camera {}
    }

    pub fn shoot(&self) -> NamedTempFile {
        let jpeg_file = NamedTempFile::new().unwrap();
        debug!("saving jpeg to {:?}", jpeg_file.path());
        let status = Command::new("gphoto2")
            .arg("--filename").arg(jpeg_file.path())
            .arg("--force-overwrite")
            //.arg("--auto-detect")
            // Choice: 1 Memory card
            .arg("--set-config").arg("capturetarget=1")
            .arg("--keep-raw")
            // Choice: 8 RAW + Large Fine JPEG
            .arg("--set-config").arg("imageformat=8")
            // Choice: 37 1/160
            .arg("--set-config").arg("shutterspeed=37")
            // Choice: 1 100
            .arg("--set-config").arg("iso=1")
            .arg("--capture-image-and-download")
            .status()
            .expect("failed to execute gphoto2");
        assert!(status.success());

        // convert IMG_3332.JPG -gravity center -extent 900x900 -crop 900x900^ -auto-level out.jpeg
        let fits_file = NamedTempFile::new().unwrap();
        debug!("saving fits to {:?}", fits_file.path());
        let status = Command::new("convert")
            .arg(format!("jpeg:{}", jpeg_file.path().to_str().unwrap()))
            .arg("-gravity").arg("center")
            .arg("-extent").arg("900x900")
            .arg("-crop").arg("900x900")
            .arg("-set").arg("colorspace").arg("Gray")
            .arg("-separate")
            .arg("-average")
            .arg("-depth").arg("16")
            .arg(format!("fits:{}", fits_file.path().to_str().unwrap()))
            .status()
            .expect("failed to execute convert");
        assert!(status.success());

        fits_file
    }
}
