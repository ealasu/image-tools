use std::f32;
use geom::{Point, Matrix3x3};

/// Returns the matrix that transforms the triangle `src` to `dst`.
pub fn get_transform_matrix(dst: [Point<f32>; 3], src: [Point<f32>; 3]) -> Matrix3x3<f64> {
    fn poly_to_matrix(points: [Point<f32>; 3]) -> Matrix3x3<f64> {
        Matrix3x3 {
            v11: points[0].x,
            v21: points[0].y,
            v31: 1.0,
            v12: points[1].x,
            v22: points[1].y,
            v32: 1.0,
            v13: points[2].x,
            v23: points[2].y,
            v33: 1.0,
        }.to_f64()
    }
    let res = poly_to_matrix(dst) * poly_to_matrix(src).inverse();
    assert!(!res.has_nan(), "matrix has nan: {:?}", res);
    res
}

/// Iterates through a number of corresponding triangle pairs, calculates transform for
/// each pair, then returns the best transform.
pub fn align_simple(matching_stars: &[(Point<f32>, Point<f32>)]) -> Matrix3x3<f64> {
    //println!("found match");
    let mut best_tx = Default::default();
    let mut best_err = f32::MAX;
    //let mut best_points = None;
    for w in matching_stars.windows(3) {
        let tx = get_transform_matrix(
            [w[0].1, w[1].1, w[2].1],
            [w[0].0, w[1].0, w[2].0]);
        let err: f32 = matching_stars.iter()
            .map(|&(r_o, s_o)| {
                let tx_r = (tx * r_o.to_f64()).to_f32();
                (tx_r.x - s_o.x).powi(2) + (tx_r.y - s_o.y).powi(2)
            })
            .sum();
        //println!("err: {}", err);
        if err < best_err {
            best_tx = tx;
            best_err = err;
            //best_points = Some((
            //[w[0].1, w[1].1, w[2].1],
            //[w[0].0, w[1].0, w[2].0]));
        }
    }
    println!("best err: {}", best_err);
    //println!("best points: {:?}", best_points);
    best_tx
}

/// From https://igl.ethz.ch/projects/ARAP/svd_rot.pdf
pub fn align_all(matching_stars: &[(Point<f32>, Point<f32>)]) -> Matrix3x3<f64> {
    unimplemented!()
}

//fn centroid(points: &[Point<f32>]) -> Point<f32> {
//}
