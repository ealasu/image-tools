use std::collections::BTreeMap;
use point::*;

pub type Star = Point<f32>;
pub type Stars = Vec<Star>;
pub type ImagesWithStars = BTreeMap<String, Stars>;
pub type ImagesWithAlignment = BTreeMap<String, Vector>;
