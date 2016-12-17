use tempfile::NamedTempFile;
use std::process::Command;

pub struct Camera {
}

impl Camera {
    pub fn new() -> Self {
        Camera {}
    }

    pub fn shoot(&self) -> NamedTempFile {
        let f = NamedTempFile::new().unwrap();
        let status = Command::new("gphoto2")
            //.arg("--filename").arg(f.path())
            .arg("--set-config").arg("shutterspeed=20")
            .arg("--set-config").arg("iso=3200")
            .arg("--set-config").arg("imageformat=raw+jpeg")
            .arg("--capture-image-and-download")
            .status()
            .expect("failed to execute gphoto2");
        assert!(status.success());
        f
    }
}
