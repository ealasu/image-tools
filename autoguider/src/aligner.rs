use pos::Vector;
use donuts;
use image::Image;

pub struct Aligner {
    reference_image: Option<donuts::Projection>,
}

impl Aligner {
    pub fn new() -> Self {
        Aligner {
            reference_image: None,
        }
    }

    pub fn align(&mut self, image: Image<f32>) -> Vector {
        let processed_image = donuts::preprocess_image(image);
        if let Some(ref reference_image) = self.reference_image {
            let (x, y) = donuts::align(reference_image, &processed_image);
            Vector {
                x: x as f64,
                y: y as f64,
            }
        } else {
            self.reference_image = Some(processed_image);
            Vector {
                x: 0.0,
                y: 0.0,
            }
        }
    }
}
