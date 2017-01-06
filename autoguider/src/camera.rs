use tempfile::NamedTempFile;
use std::process::Command;
use std::fs;
use std::path::Path;
use image::{Image, Rgb};
use gphoto;

pub struct Camera {
    pub keep_raw: bool,
    //context: gphoto::Context,
    //camera: gphoto::Camera,
}

unsafe impl Send for Camera {}

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

        //let mut context = gphoto::Context::new().unwrap();
        //let camera = gphoto::Camera::autodetect(&mut context).unwrap();

        Camera {
            keep_raw: true,
            //context: context,
            //camera: camera,
        }
    }

    pub fn shoot(&self) -> Image<f32> {
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
             //Choice: 37 1/160
            //.arg("--set-config").arg("shutterspeed=37")
             //Choice: 1 100
            //.arg("--set-config").arg("iso=1")

            // 15 
            //.arg("--set-config").arg("shutterspeed=3")

            // 30
            .arg("--set-config").arg("shutterspeed=0")

            // 5000
            //.arg("--set-config").arg("iso=18")

            // 3200
            .arg("--set-config").arg("iso=16")

            .arg("--capture-image-and-download");

        let status = command
            .status()
            .expect("failed to execute gphoto2");
        assert!(status.success());
        debug!("jpeg file len: {}", fs::metadata(jpeg_file.path()).unwrap().len());
        fs::copy(jpeg_file.path(), tmpdir.join(Path::new("latest.jpeg"))).unwrap();

        let image = Image::<Rgb<f32>>::open_jpeg_file(jpeg_file.path());
        image.center_crop(900, 900).to_gray()
    }

    //pub fn shoot(&mut self) -> Image<f32> {
        //let res = self.camera.capture_image(&mut self.context);
        //if let Err(ref e) = res {
            //println!(" (error) {:?}", e.message());
        //}
        //let capture = res.unwrap();
        //println!(" (done) {:?}", capture.basename());

        //let tmpdir = Path::new("/mnt/ramdisk");
        //let jpeg_file = NamedTempFile::new_in(tmpdir).unwrap();
        //debug!("saving jpeg to {:?}", jpeg_file.path());
        //let mut file = gphoto::FileMedia::create(jpeg_file.path()).unwrap();
        //self.camera.download(&mut self.context, &capture, &mut file).unwrap();
        //fs::copy(jpeg_file.path(), tmpdir.join(Path::new("latest.jpeg"))).unwrap();

        //let image = Image::<Rgb<f32>>::open_jpeg_file(jpeg_file.path());
        //image.center_crop(900, 900).to_gray()
    //}
}
