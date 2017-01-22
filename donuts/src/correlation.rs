use std::f32;

pub struct Point {
    pub x: f32,
    pub y: f32,
}

/// Calculates how far you'd need to shift `sample` for it to
/// line up with `reference`, with a max offset of `n`.
pub fn calc_offset(reference: &[f32], sample: &[f32], n: usize) -> f32 {
    let corr = correlation(reference, sample, n);
    let dev = deviation(&corr);
    info!("deviation: {}", dev);
    correlation_peak(&corr[..])
}

/// Calculates the correlation between `reference` and `sample`, with a window of `[-n, n]`.
pub fn correlation(reference: &[f32], sample: &[f32], n: usize) -> Vec<f32> {
    assert_eq!(reference.len(), sample.len());
    let mut res = Vec::with_capacity(n * 2);
    let mut r_start = 0;
    let mut s_start = n;
    let len = reference.len() - n;
    {
        let mut push_sum = |r_start, s_start| {
            let mut sum = 0.0;
            for i in 0..len {
                sum += reference[r_start + i] * sample[s_start + i];
            }
            res.push(sum);
        };
        loop {
            push_sum(r_start, s_start);
            r_start += 1;
            if r_start > n {
                break;
            }

            push_sum(r_start, s_start);
            if s_start == 0 {
                break;
            }
            s_start -= 1;
        }
    }
    assert_eq!(res.len(), n*2+1);
    res
}

/// Calculates the peak of the correlation, to sub-pixel precision.
pub fn correlation_peak(correlation: &[f32]) -> f32 {
    let peak_pos = pos_of_max(correlation);
    let n_corr = (correlation.len() - 1) / 2;
    let peak_offset_estimate = peak_pos as isize - n_corr as isize;
    if peak_pos == 0 || peak_pos == correlation.len() - 1 {
        return peak_offset_estimate as f32;
    }
    interpolate_peak_pos([
        correlation[peak_pos - 1],
        correlation[peak_pos],
        correlation[peak_pos + 1],
    ], peak_offset_estimate as f32)
}

pub fn interpolate_peak_pos(values: [f32; 3], peak_offset_estimate: f32) -> f32 {
    let vertex = parabola_vertex(
        Point {
            x: peak_offset_estimate - 1.0,
            y: values[0]
        },
        Point {
            x: peak_offset_estimate,
            y: values[1]
        },
        Point {
            x: peak_offset_estimate + 1.0,
            y: values[2]
        });
    vertex.x
}

/// Calculates height of next tallest peak relative to tallest peak.
/// Small values mean the tallest peak is very prominent, large values mean
/// it's not so prominent.
fn deviation(correlation: &[f32]) -> f32 {
    let peak_pos = pos_of_max(correlation);
    let peak = correlation[peak_pos];
    let next_peak = next_peak(correlation, peak_pos);
    let min = correlation.iter().min_by(|a,b| a.partial_cmp(b).unwrap()).unwrap();
    (next_peak - min) / (peak - min)
}

/// Calculates absolute height of next tallest peak
fn next_peak(data: &[f32], peak_pos: usize) -> f32 {
    let mut last_v = data[peak_pos];
    let mut peak_left = peak_pos;
    loop {
        let v = data[peak_left];
        if v > last_v {
            break;
        }
        last_v = v;
        if peak_left == 0 {
            break;
        }
        peak_left -= 1;
    }

    let mut last_v = data[peak_pos];
    let mut peak_right = peak_pos;
    loop {
        let v = data[peak_right];
        if v > last_v {
            break;
        }
        last_v = v;
        if peak_right >= data.len() - 1 {
            break;
        }
        peak_right += 1;
    }

    *data[0..peak_left].iter()
        .chain(data[peak_right..].iter())
        .max_by(|a,b| a.partial_cmp(b).unwrap()).unwrap()
}

pub fn pos_of_max(slice: &[f32]) -> usize {
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

pub fn parabola_vertex(p1: Point, p2: Point, p3: Point) -> Point {
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
    use rand::{self, Rng};

    #[test]
    fn test_correlation() {
        let r = [0.0, 1.0, 2.0, 3.0, 2.0, 1.0, 0.0, 0.0];
        let s = [1.0, 0.0, 1.0, 2.0, 4.0, 2.0, 1.0, 0.0];
        let res = correlation(&r[..], &s[..], 3);
        //let expected = [11.0, 18.0, 22.0, 18.0, 12.0, 6.0, 4.0];
        let expected = [
            s[3]*r[0] + s[4]*r[1] + s[5]*r[2] + s[6]*r[3] + s[7]*r[4], // -3
            s[3]*r[1] + s[4]*r[2] + s[5]*r[3] + s[6]*r[4] + s[7]*r[5], // -2
            s[2]*r[1] + s[3]*r[2] + s[4]*r[3] + s[5]*r[4] + s[6]*r[5], // -1
            s[2]*r[2] + s[3]*r[3] + s[4]*r[4] + s[5]*r[5] + s[6]*r[6], // 0
            s[1]*r[2] + s[2]*r[3] + s[3]*r[4] + s[4]*r[5] + s[5]*r[6], // 1
            s[1]*r[3] + s[2]*r[4] + s[3]*r[5] + s[4]*r[6] + s[5]*r[7], // 2
            s[0]*r[3] + s[1]*r[4] + s[2]*r[5] + s[3]*r[6] + s[4]*r[7], // 3
        ];
        assert_eq!(res[..], expected[..]);
    }

    #[test]
    fn test_correlation_2() {
        let r = [0.0, 1.0, 2.0, 3.0, 2.0, 5.0, 0.0, 0.0];
        let s = [1.0, 0.0, 1.0, 2.0, 4.0, 2.0, 1.0, 0.0];
        let res = correlation(&r[..], &s[..], 3);
        //let expected = [11.0, 18.0, 22.0, 18.0, 12.0, 6.0, 4.0];
        let expected = [
            s[3]*r[0] + s[4]*r[1] + s[5]*r[2] + s[6]*r[3] + s[7]*r[4], // -3
            s[3]*r[1] + s[4]*r[2] + s[5]*r[3] + s[6]*r[4] + s[7]*r[5], // -2
            s[2]*r[1] + s[3]*r[2] + s[4]*r[3] + s[5]*r[4] + s[6]*r[5], // -1
            s[2]*r[2] + s[3]*r[3] + s[4]*r[4] + s[5]*r[5] + s[6]*r[6], // 0
            s[1]*r[2] + s[2]*r[3] + s[3]*r[4] + s[4]*r[5] + s[5]*r[6], // 1
            s[1]*r[3] + s[2]*r[4] + s[3]*r[5] + s[4]*r[6] + s[5]*r[7], // 2
            s[0]*r[3] + s[1]*r[4] + s[2]*r[5] + s[3]*r[6] + s[4]*r[7], // 3
        ];
        assert_eq!(res[..], expected[..]);
    }

    #[test]
    fn test_correlation_odd_data_len() {
        let r = [0.0, 1.0, 2.0, 3.0, 2.0, 1.0, 0.0, 0.0, 1.0];
        let s = [1.0, 0.0, 1.0, 2.0, 4.0, 2.0, 1.0, 0.0, -1.0];
        let res = correlation(&r[..], &s[..], 3);
        //let expected = [11.0, 18.0, 22.0, 18.0, 12.0, 6.0, 4.0];
        let expected = [
            s[3]*r[0] + s[4]*r[1] + s[5]*r[2] + s[6]*r[3] + s[7]*r[4] + s[8]*r[5], // -3
            s[3]*r[1] + s[4]*r[2] + s[5]*r[3] + s[6]*r[4] + s[7]*r[5] + s[8]*r[6], // -2
            s[2]*r[1] + s[3]*r[2] + s[4]*r[3] + s[5]*r[4] + s[6]*r[5] + s[7]*r[6], // -1
            s[2]*r[2] + s[3]*r[3] + s[4]*r[4] + s[5]*r[5] + s[6]*r[6] + s[7]*r[7], // 0
            s[1]*r[2] + s[2]*r[3] + s[3]*r[4] + s[4]*r[5] + s[5]*r[6] + s[6]*r[7], // 1
            s[1]*r[3] + s[2]*r[4] + s[3]*r[5] + s[4]*r[6] + s[5]*r[7] + s[6]*r[8], // 2
            s[0]*r[3] + s[1]*r[4] + s[2]*r[5] + s[3]*r[6] + s[4]*r[7] + s[5]*r[8], // 3
        ];
        assert_eq!(res[..], expected[..]);
    }

    #[test]
    fn test_correlation_even_n() {
        let r = [0.0, 1.0, 2.0, 3.0, 2.0, 1.0, 0.0, 0.0];
        let s = [1.0, 0.0, 1.0, 2.0, 4.0, 2.0, 1.0, 0.0];
        let res = correlation(&r[..], &s[..], 2);
        let expected = [
            s[2]*r[0] + s[3]*r[1] + s[4]*r[2] + s[5]*r[3] + s[6]*r[4], // -2
            s[2]*r[1] + s[3]*r[2] + s[4]*r[3] + s[5]*r[4] + s[6]*r[5], // -1
            s[1]*r[1] + s[2]*r[2] + s[3]*r[3] + s[4]*r[4] + s[5]*r[5], // 0
            s[1]*r[2] + s[2]*r[3] + s[3]*r[4] + s[4]*r[5] + s[5]*r[6], // 1
            s[0]*r[2] + s[1]*r[3] + s[2]*r[4] + s[3]*r[5] + s[4]*r[6], // 2
        ];
        assert_eq!(res[..], expected[..]);
    }

    #[test]
    fn test_zero() {
        let corr = [0.0, 1.0, 2.0, 1.0, 0.0];
        let x = correlation_peak(&corr[..]);
        assert_eq!(x, 0.0);
    }

    #[test]
    fn test_one() {
        let corr = [1.0, 2.0, 1.0, 0.0, 1.0];
        let x = correlation_peak(&corr[..]);
        assert_eq!(x, -1.0);
    }

    #[test]
    fn test_fraction() {
        let corr = [1.0, 2.0, 2.0, 1.0, 0.0];
        let x = correlation_peak(&corr[..]);
        assert_eq!(x, -0.5);
    }
}
