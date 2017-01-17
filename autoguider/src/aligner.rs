use geom::Vector;
use donuts;
use image::Image;

pub struct Aligner {
    reference_image: Option<donuts::projection::Projection>,
}

impl Aligner {
    pub fn new() -> Self {
        Aligner {
            reference_image: None,
        }
    }

    pub fn align(&mut self, image: Image<f32>) -> Vector<f32> {
        let processed_image = donuts::preprocess_image(image);
        if let Some(ref reference_image) = self.reference_image {
            donuts::align(reference_image, &processed_image, 200)
        } else {
            self.reference_image = Some(processed_image);
            Vector {
                x: 0.0,
                y: 0.0,
            }
        }
    }
}
