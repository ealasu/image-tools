use unit::Unit;
use std::ops::*;

#[derive(Copy, Clone, Debug)]
pub struct Matrix3x1 {
    pub v11: Unit,
    pub v21: Unit,
    pub v31: Unit,
}

impl Matrix3x1 {
    pub fn point(x: f32, y: f32) -> Self {
        Matrix3x1 {
            v11: x,
            v21: y,
            v31: 0.0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Matrix3x3 {
    pub v11: Unit,
    pub v12: Unit,
    pub v13: Unit,
    pub v21: Unit,
    pub v22: Unit,
    pub v23: Unit,
    pub v31: Unit,
    pub v32: Unit,
    pub v33: Unit,
}

impl Matrix3x3 {
    pub fn translation(x: f32, y: f32) -> Self {
        Matrix3x3 {
            v11: 1.0, v12: 0.0, v13: x,
            v21: 0.0, v22: 1.0, v23: y,
            v31: 0.0, v32: 0.0, v33: 1.0,
        }
    }

    pub fn rotation(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Matrix3x3 {
            v11: cos, v12: -sin, v13: 0.0,
            v21: sin, v22: cos,  v23: 0.0,
            v31: 0.0, v32: 0.0,  v33: 1.0,
        }
    }
}

impl Mul for Matrix3x3 {
    type Output = Matrix3x3;
    fn mul(self, rhs: Self) -> Self::Output {
        Matrix3x3 {
            v11: self.v11 * rhs.v11 + self.v12 * rhs.v21 + self.v13 * rhs.v31,
            v21: self.v21 * rhs.v11 + self.v22 * rhs.v21 + self.v23 * rhs.v31,
            v31: self.v31 * rhs.v11 + self.v32 * rhs.v21 + self.v33 * rhs.v31,

            v12: self.v11 * rhs.v12 + self.v12 * rhs.v22 + self.v13 * rhs.v32,
            v22: self.v21 * rhs.v12 + self.v22 * rhs.v22 + self.v23 * rhs.v32,
            v32: self.v31 * rhs.v12 + self.v32 * rhs.v22 + self.v33 * rhs.v32,

            v13: self.v11 * rhs.v13 + self.v12 * rhs.v23 + self.v13 * rhs.v33,
            v23: self.v21 * rhs.v13 + self.v22 * rhs.v23 + self.v23 * rhs.v33,
            v33: self.v31 * rhs.v13 + self.v32 * rhs.v23 + self.v33 * rhs.v33,
        }
    }
}

impl Mul<Matrix3x1> for Matrix3x3 {
    type Output = Matrix3x1;
    fn mul(self, rhs: Matrix3x1) -> Self::Output {
        Matrix3x1 {
            v11: self.v11 * rhs.v11 + self.v12 * rhs.v21 + self.v13 * rhs.v31,
            v21: self.v21 * rhs.v11 + self.v22 * rhs.v21 + self.v23 * rhs.v31,
            v31: self.v31 * rhs.v11 + self.v32 * rhs.v21 + self.v33 * rhs.v31,
        }
    }
}
