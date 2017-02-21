use tempfile::NamedTempFile;
use std::process::Command;
use std::fs;
use std::path::Path;
use image::{Image, Rgb};
use gphoto;

pub struct Camera {
    //context: gphoto::Context,
    //camera: gphoto::Camera,
}

unsafe impl Send for Camera {}

impl Camera {
    pub fn new() -> Self {
        //let status = Command::new("umount")
            //.arg("/mnt/ramdisk")
            //.status()
            //.expect("failed to execute umount");
        //assert!(status.success());
        //let status = Command::new("mount")
            //.arg("-a")
            //.status()
            //.expect("failed to execute mount");
        //assert!(status.success());

        //let mut context = gphoto::Context::new().unwrap();
        //let camera = gphoto::Camera::autodetect(&mut context).unwrap();

        Camera {
            //context: context,
            //camera: camera,
        }
    }

    pub fn shoot(&self) -> gphoto::Result<Image<f32>> {
        let jpeg_file = gphoto::shoot(gphoto::Options {
            keep_raw: true,
            shutter_speed: "0".into(), // 30s
            iso: "16".into(), // ISO 3200?
        })?;
        let image = Image::<Rgb<u8>>::open_jpeg_file(jpeg_file.path()).to_f32();
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
