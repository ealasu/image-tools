use projection::Projection;
use correlation;
use geom::Vector;

pub fn align(reference: &Projection, sample: &Projection) -> Vector {
    let n = 200;
    let x = correlation::calc_offset(&reference.x[..], &sample.x[..], n);
    let y = correlation::calc_offset(&reference.y[..], &sample.y[..], n);
    Vector { x: x, y: y }
}
