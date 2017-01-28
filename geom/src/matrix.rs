use std::ops::*;
use std::fmt::Display;
use num::Float;
use point::Point;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Matrix3x1<T: Float> {
    pub v11: T,
    pub v21: T,
    pub v31: T,
}

impl<T: Float> Matrix3x1<T> {
    pub fn from_point(point: &Point<T>) -> Self {
        Matrix3x1 {
            v11: point.x,
            v21: point.y,
            v31: T::one(),
        }
    }

    pub fn to_point(&self) -> Point<T> {
        Point {
            x: self.v11,
            y: self.v11,
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Matrix3x3<T: Float> {
    pub v11: T,
    pub v12: T,
    pub v13: T,
    pub v21: T,
    pub v22: T,
    pub v23: T,
    pub v31: T,
    pub v32: T,
    pub v33: T,
}

impl<T: Float+Display> Matrix3x3<T> {
    pub fn to_f64(&self) -> Matrix3x3<f64> {
        Matrix3x3 {
            v11: self.v11.to_f64().unwrap(),
            v12: self.v12.to_f64().unwrap(),
            v13: self.v13.to_f64().unwrap(),
            v21: self.v21.to_f64().unwrap(),
            v22: self.v22.to_f64().unwrap(),
            v23: self.v23.to_f64().unwrap(),
            v31: self.v31.to_f64().unwrap(),
            v32: self.v32.to_f64().unwrap(),
            v33: self.v33.to_f64().unwrap(),
        }
    }

    pub fn identity() -> Self {
        Matrix3x3 {
            v11: T::one(), v12: T::zero(), v13: T::zero(),
            v21: T::zero(), v22: T::one(), v23: T::zero(),
            v31: T::zero(), v32: T::zero(), v33: T::one(),
        }
    }

    pub fn translation(x: T, y: T) -> Self {
        Matrix3x3 {
            v11: T::one(), v12: T::zero(), v13: x,
            v21: T::zero(), v22: T::one(), v23: y,
            v31: T::zero(), v32: T::zero(), v33: T::one(),
        }
    }

    pub fn rotation(angle: T) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Matrix3x3 {
            v11: cos, v12: -sin, v13: T::zero(),
            v21: sin, v22: cos,  v23: T::zero(),
            v31: T::zero(), v32: T::zero(),  v33: T::one(),
        }
    }

    pub fn has_nan(&self) -> bool {
        self.v11.is_nan() ||
        self.v12.is_nan() ||
        self.v13.is_nan() ||
        self.v21.is_nan() ||
        self.v22.is_nan() ||
        self.v23.is_nan() ||
        self.v31.is_nan() ||
        self.v32.is_nan() ||
        self.v33.is_nan()
    }

    pub fn inverse(&self) -> Self {
        let A=(self.v22 * self.v33-self.v23 * self.v32);
        let D=-(self.v12 * self.v33-self.v13 * self.v32);
        let G=(self.v12 * self.v23-self.v13 * self.v22);
        let B=-(self.v21 * self.v33-self.v23 * self.v31);
        let E=(self.v11 * self.v33-self.v13 * self.v31);
        let H=-(self.v11 * self.v23-self.v13 * self.v21); 
        let C=(self.v21 * self.v32-self.v22 * self.v31);
        let F=-(self.v11 * self.v32-self.v12 * self.v31);
        let I=(self.v11 * self.v22-self.v12 * self.v21);
        let det = self.v11 * A + self.v12 * B + self.v13 * C;
        let m = T::one() / det;
        Matrix3x3 {
            v11: m*A, v12: m*D, v13: m*G,
            v21: m*B, v22: m*E, v23: m*H,
            v31: m*C, v32: m*F, v33: m*I,
        }
    }
}

impl<T: Float> Mul for Matrix3x3<T> {
    type Output = Matrix3x3<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        Matrix3x3 {
            v11: self.v11 * rhs.v11 + self.v12 * rhs.v21 + self.v13 * rhs.v31,
            v12: self.v11 * rhs.v12 + self.v12 * rhs.v22 + self.v13 * rhs.v32,
            v13: self.v11 * rhs.v13 + self.v12 * rhs.v23 + self.v13 * rhs.v33,

            v21: self.v21 * rhs.v11 + self.v22 * rhs.v21 + self.v23 * rhs.v31,
            v22: self.v21 * rhs.v12 + self.v22 * rhs.v22 + self.v23 * rhs.v32,
            v23: self.v21 * rhs.v13 + self.v22 * rhs.v23 + self.v23 * rhs.v33,

            v31: self.v31 * rhs.v11 + self.v32 * rhs.v21 + self.v33 * rhs.v31,
            v32: self.v31 * rhs.v12 + self.v32 * rhs.v22 + self.v33 * rhs.v32,
            v33: self.v31 * rhs.v13 + self.v32 * rhs.v23 + self.v33 * rhs.v33,
        }
    }
}

impl<T: Float> Mul<Matrix3x1<T>> for Matrix3x3<T> {
    type Output = Matrix3x1<T>;
    fn mul(self, rhs: Matrix3x1<T>) -> Self::Output {
        Matrix3x1 {
            v11: self.v11 * rhs.v11 + self.v12 * rhs.v21 + self.v13 * rhs.v31,
            v21: self.v21 * rhs.v11 + self.v22 * rhs.v21 + self.v23 * rhs.v31,
            v31: self.v31 * rhs.v11 + self.v32 * rhs.v21 + self.v33 * rhs.v31,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let m = Matrix3x3 {
            v11: 1.0, v12: 0.0, v13: -2748.0,
            v21: 0.0, v22: 1.0, v23: -1835.0,
            v31: 0.0, v32: 0.0, v33: 1.0
        };
        let p = Matrix3x1::point(
            5.0,
            10.0,
        );
        let res = m * p;
        println!("res: {:?}", res);
    }

    //#[test]
    //fn invert() {
        //let m = Matrix3x3 {
            //v11: 1.0, v12: 2.0, v13: 4.0,
            //v21: 4.0, v22: 5.0, v23: 6.0,
            //v31: 7.0, v32: 8.0, v33: 9.0,
        //};
        //let inv = m.inverse();
        //let inv_inv = inv.inverse();
        //assert_eq!(inv_inv, m);
    //}
}
