use std::f32;

struct Point {
    x: f32,
    y: f32,
}

pub fn calc_offset(correlation: &[f32]) -> f32 {
    let peak_pos = pos_of_max(correlation);
    let n_corr = (correlation.len() - 1) / 2;
    let peak_offset_estimate = peak_pos as isize - n_corr as isize;
    let vertex = parabola_vertex(
        Point {
            x: (peak_offset_estimate - 1) as f32,
            y: correlation[peak_pos - 1]
        },
        Point {
            x: peak_offset_estimate as f32,
            y: correlation[peak_pos]
        },
        Point {
            x: (peak_offset_estimate + 1) as f32,
            y: correlation[peak_pos + 1]
        });
    vertex.x
}

fn pos_of_max(slice: &[f32]) -> usize {
    let mut max_pos = 0;
    let mut max = f32::NEG_INFINITY;
    for (i, item) in slice.iter().enumerate() {
        if *item > max {
            max = *item;
            max_pos = i;
        }
    }
    max_pos
}

fn parabola_vertex(p1: Point, p2: Point, p3: Point) -> Point {
    let denom = (p1.x - p2.x) * (p1.x - p3.x) * (p2.x - p3.x);
    let a     = (p3.x * (p2.y - p1.y) + p2.x * (p1.y - p3.y) + p1.x * (p3.y - p2.y)) / denom;
    let b     = (p3.x*p3.x * (p1.y - p2.y) + p2.x*p2.x * (p3.y - p1.y) + p1.x*p1.x * (p2.y - p3.y)) / denom;
    let c     = (p2.x * p3.x * (p2.x - p3.x) * p1.y + p3.x * p1.x * (p3.x - p1.x) * p2.y + p1.x * p2.x * (p1.x - p2.x) * p3.y) / denom;

    Point {
        x: -b / (2.0 * a),
        y: c - b * b / (4.0 * a),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_zero() {
        let corr = [0.0, 1.0, 2.0, 1.0, 0.0];
        let x = calc_offset(&corr[..]);
        assert_eq!(x, 0.0);
    }

    #[test]
    fn test_one() {
        let corr = [1.0, 2.0, 1.0, 0.0, 1.0];
        let x = calc_offset(&corr[..]);
        assert_eq!(x, -1.0);
    }

    #[test]
    fn test_fraction() {
        let corr = [1.0, 2.0, 2.0, 1.0, 0.0];
        let x = calc_offset(&corr[..]);
        assert_eq!(x, -0.5);
    }
}
