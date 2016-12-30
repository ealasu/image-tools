pub fn correlation(reference: &[f32], sample: &[f32], n: usize) -> Vec<f32> {
    assert_eq!(reference.len(), sample.len());
    let mut res = Vec::with_capacity(n * 2);
    let n = n as isize;
    for offset in -n..n+1 {
        let len = reference.len() - offset.abs() as usize;
        let s_start = if offset < 0 { -offset } else { 0 } as usize;
        let r_start = if offset < 0 { 0 } else { offset } as usize;
        let mut sum = 0.0;
        for i in 0..len {
            sum += reference[r_start + i] * sample[s_start + i];
        }
        res.push(sum);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use rand::{self, Rng};

    #[test]
    fn test() {
        let reference = [0.0, 1.0, 2.0, 3.0, 2.0, 1.0, 0.0, 0.0];
        let sample =    [1.0, 0.0, 1.0, 2.0, 4.0, 2.0, 1.0, 0.0];
        let res = correlation(&reference[..], &sample[..], 3);
        let expected = [11.0, 18.0, 22.0, 18.0, 12.0, 6.0, 4.0];
        assert_eq!(res[..], expected[..]);
    }
}
