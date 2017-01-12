use unit::Unit;
use std::ops::*;
use num::Float;

#[derive(Copy, Clone, Debug)]
pub struct Matrix3x1<T: Float> {
    pub v11: T,
    pub v21: T,
    pub v31: T,
}

impl<T: Float> Matrix3x1<T> {
    pub fn point(x: T, y: T) -> Self {
        Matrix3x1 {
            v11: x,
            v21: y,
            v31: T::one(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
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

impl<T: Float> Matrix3x3<T> {
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
}
