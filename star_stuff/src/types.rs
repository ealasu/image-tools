use std::collections::BTreeMap;
use point::*;

pub type Star = Point<f32>;
pub type Stars = Vec<Star>;
pub type ImagesWithStars = Vec<(String, Stars)>;
pub type ImagesWithAlignment = Vec<(String, Vector)>;
