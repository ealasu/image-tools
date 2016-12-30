use image::Image;

pub fn x_projection(image: &Image<f32>) -> Vec<f32> {
    let mut res = Vec::with_capacity(image.height);
    let height = image.height as f32;
    for x in 0..image.width {
        let mut sum = 0.0;
        for y in 0..image.height {
            sum += *image.pixel_at(x, y);
        }
        res.push(sum / height);
    }
    res
}

pub fn y_projection(image: &Image<f32>) -> Vec<f32> {
    let mut res = Vec::with_capacity(image.width);
    let width = image.width as f32;
    for y in 0..image.height {
        let mut sum = 0.0;
        for x in 0..image.width {
            sum += *image.pixel_at(x, y);
        }
        res.push(sum / width);
    }
    res
}
