use std::f64;
use geom::{Point, Matrix3x3};
//use ndarray::prelude::*;
//use ndarray_linalg::prelude::*;
use rulinalg::matrix::{Matrix, BaseMatrix};
use rulinalg::vector::Vector;

/// Returns the matrix that transforms the triangle `src` to `dst`.
pub fn get_transform_matrix(dst: [Point<f64>; 3], src: [Point<f64>; 3]) -> Matrix3x3<f64> {
    fn poly_to_matrix(points: [Point<f64>; 3]) -> Matrix3x3<f64> {
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

fn calc_err(matching_stars: &[(Point<f64>, Point<f64>)], tx: Matrix3x3<f64>) -> f64 {
    matching_stars.iter()
        .map(|&(r_o, s_o)| {
            ((tx * r_o) - s_o).length2()
        })
        .sum()
}

/// Iterates through a number of corresponding triangle pairs, calculates transform for
/// each pair, then returns the best transform.
pub fn align_simple(matching_stars: &[(Point<f64>, Point<f64>)]) -> Matrix3x3<f64> {
    //println!("found match");
    let mut best_tx = Default::default();
    let mut best_err = f64::MAX;
    //let mut best_points = None;
    for w in matching_stars.windows(3) {
        let tx = get_transform_matrix(
            [w[0].1, w[1].1, w[2].1],
            [w[0].0, w[1].0, w[2].0]);
        let err = calc_err(matching_stars, tx);
        //println!("err: {}", err);
        if err < best_err {
            best_tx = tx;
            best_err = err;
            //best_points = Some((
            //[w[0].1, w[1].1, w[2].1],
            //[w[0].0, w[1].0, w[2].0]));
        }
    }
    //println!("best err: {}", best_err);
    //println!("best points: {:?}", best_points);
    best_tx
}

/// From https://igl.ethz.ch/projects/ARAP/svd_rot.pdf
pub fn align_all(matching_stars: &[(Point<f64>, Point<f64>)]) -> Matrix3x3<f64> {
    let ref_points = matching_stars.iter().map(|&(r,_)| r).collect::<Vec<_>>();
    let sam_points = matching_stars.iter().map(|&(_,s)| s).collect::<Vec<_>>();
    let ref_centroid = centroid(&ref_points);
    let sam_centroid = centroid(&sam_points);

    //let mut m = Array::from_elem((2, 2), 0.0f64);
    let mut m = Matrix::<f64>::zeros(2, 2);
    for &(r, s) in matching_stars.iter() {
        let r = r - ref_centroid;
        let s = s - sam_centroid;
        m[[0, 0]] += r.x * s.x;
        m[[0, 1]] += r.x * s.y;
        m[[1, 0]] += r.y * s.x;
        m[[1, 1]] += r.y * s.y;
        //*m.get_mut((0, 0)).unwrap() += r.x * s.x;
        //*m.get_mut((0, 1)).unwrap() += r.x * s.y;
        //*m.get_mut((1, 0)).unwrap() += r.y * s.x;
        //*m.get_mut((1, 1)).unwrap() += r.y * s.y;
    }
    println!("m:    {:?}", m);
    let (s, u, v) = m.svd().expect("failed to calculate SVD");
    println!("s: {:?}", s);
    println!("u: {:?}", u);
    println!("v: {:?}", v);
    println!("usvt: {:?}", &u * &s * &v.transpose());
    let d = Matrix::from_diag(&vec![1.0, (&v * u.transpose()).det()]);
    let r = v * d * u.transpose();
    //println!("r: {:?}", r);
    let t =
        Vector::new(vec![sam_centroid.x, sam_centroid.y])
        - &r *
        Vector::new(vec![ref_centroid.x, ref_centroid.y]);
    //println!("t: {:?}", t);

    let tx = Matrix3x3 {
        v11: r[[0, 0]], v12: r[[0, 1]], v13: t[0],
        v21: r[[1, 0]], v22: r[[1, 1]], v23: t[1],
        v31: 0.0, v32: 0.0, v33: 1.0,
    };
    let err = calc_err(matching_stars, tx);
    //println!("err: {}", err);

    tx
}

fn centroid(points: &[Point<f64>]) -> Point<f64> {
    let mut sum: Point<f64> = Default::default();
    for &p in points.iter() {
        sum += p;
    }
    sum / points.len() as f64
}
