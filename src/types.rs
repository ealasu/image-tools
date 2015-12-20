use std::collections::BTreeMap;
use point::Point;

pub type Star = Point<f32>;
pub type Stars = Vec<Star>;
pub type ImagesWithStars = BTreeMap<String, Stars>;
pub type Transform = Point<f32>;
pub type ImagesWithAlignment = BTreeMap<String, Transform>;
