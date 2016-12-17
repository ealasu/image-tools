use tempfile::NamedTempFile;
use std::process::Command;
use std::str;
use pos::Vector;

pub struct Aligner {
    reference_image: Option<NamedTempFile>,
}

impl Aligner {
    pub fn new() -> Self {
        Aligner {
            reference_image: None,
        }
    }

    pub fn align(&mut self, image: NamedTempFile) -> Vector {
        if let Some(ref reference_image) = self.reference_image {
            let output = Command::new("/Users/emi/repos/projects/donuts-test/main.py")
                                 .arg(reference_image.path())
                                 .arg(image.path())
                                 .output()
                                 .expect("failed to execute donuts");
            let s = str::from_utf8(&output.stdout).unwrap();
            let mut s = s.split(",");
            let x = s.next().unwrap();
            let x = x.trim().parse::<f32>().unwrap();
            let y = s.next().unwrap();
            let y = y.trim().parse::<f32>().unwrap();
            Vector { x: x, y: y }
        } else {
            self.reference_image = Some(image);
            Vector { x: 0.0, y: 0.0 }
        }
    }
}
