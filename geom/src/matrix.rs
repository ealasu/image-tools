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
    #[inline]
    pub fn from_point(point: Point<T>) -> Self {
        Matrix3x1 {
            v11: point.x,
            v21: point.y,
            v31: T::one(),
        }
    }

    #[inline]
    pub fn to_point(&self) -> Point<T> {
        Point {
            x: self.v11,
            y: self.v21,
        }
    }

    #[inline]
    pub fn to_f64(&self) -> Matrix3x1<f64> {
        Matrix3x1 {
            v11: self.v11.to_f64().unwrap(),
            v21: self.v21.to_f64().unwrap(),
            v31: self.v31.to_f64().unwrap(),
        }
    }
}

//#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
//pub struct Matrix1x3<T: Float> {
    //pub v11: T,
    //pub v12: T,
    //pub v13: T,
//}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
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

    pub fn to_f32(&self) -> Matrix3x3<f32> {
        Matrix3x3 {
            v11: self.v11.to_f32().unwrap(),
            v12: self.v12.to_f32().unwrap(),
            v13: self.v13.to_f32().unwrap(),
            v21: self.v21.to_f32().unwrap(),
            v22: self.v22.to_f32().unwrap(),
            v23: self.v23.to_f32().unwrap(),
            v31: self.v31.to_f32().unwrap(),
            v32: self.v32.to_f32().unwrap(),
            v33: self.v33.to_f32().unwrap(),
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

    #[allow(unused_parens)]
    pub fn inverse(&self) -> Self {
        let a=(self.v22 * self.v33-self.v23 * self.v32);
        let d=-(self.v12 * self.v33-self.v13 * self.v32);
        let g=(self.v12 * self.v23-self.v13 * self.v22);
        let b=-(self.v21 * self.v33-self.v23 * self.v31);
        let e=(self.v11 * self.v33-self.v13 * self.v31);
        let h=-(self.v11 * self.v23-self.v13 * self.v21); 
        let c=(self.v21 * self.v32-self.v22 * self.v31);
        let f=-(self.v11 * self.v32-self.v12 * self.v31);
        let i=(self.v11 * self.v22-self.v12 * self.v21);
        let det = self.v11 * a + self.v12 * b + self.v13 * c;
        let m = T::one() / det;
        Matrix3x3 {
            v11: m*a, v12: m*d, v13: m*g,
            v21: m*b, v22: m*e, v23: m*h,
            v31: m*c, v32: m*f, v33: m*i,
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

impl<T: Float> Mul<Point<T>> for Matrix3x3<T> {
    type Output = Point<T>;
    fn mul(self, rhs: Point<T>) -> Self::Output {
        (self * Matrix3x1::from_point(rhs)).to_point()
    }
}

impl<T: Float> AddAssign for Matrix3x3<T> {
    fn add_assign(&mut self, other: Self) {
        self.v11 = self.v11 + other.v11;
        self.v12 = self.v12 + other.v12;
        self.v13 = self.v13 + other.v13;
        self.v21 = self.v21 + other.v21;
        self.v22 = self.v22 + other.v22;
        self.v23 = self.v23 + other.v23;
        self.v31 = self.v31 + other.v31;
        self.v32 = self.v32 + other.v32;
        self.v33 = self.v33 + other.v33;
    }
}

impl<T: Float> DivAssign<T> for Matrix3x3<T> {
    fn div_assign(&mut self, other: T) {
        self.v11 = self.v11 / other;
        self.v12 = self.v12 / other;
        self.v13 = self.v13 / other;
        self.v21 = self.v21 / other;
        self.v22 = self.v22 / other;
        self.v23 = self.v23 / other;
        self.v31 = self.v31 / other;
        self.v32 = self.v32 / other;
        self.v33 = self.v33 / other;
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
