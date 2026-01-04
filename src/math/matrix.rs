use super::vector::Vec4;
use std::ops::Mul;

#[derive(Clone, Copy, Debug)]
pub struct Mat4 {
    pub data: [[f32; 4]; 4],
}

impl Mat4 {
    pub fn new(data: [[f32; 4]; 4]) -> Self {
        Self { data }
    }

    pub fn zero() -> Self {
        Self {
            data: [[0.0; 4]; 4],
        }
    }

    pub fn identity() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, x],
                [0.0, 1.0, 0.0, y],
                [0.0, 0.0, 1.0, z],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        Self {
            data: [
                [x, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0],
                [0.0, 0.0, z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotation_x(angle: f32) -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, angle.cos(), -angle.sin(), 0.0],
                [0.0, angle.sin(), angle.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
    pub fn rotation_y(angle: f32) -> Self {
        Self {
            data: [
                [angle.cos(), 0.0, angle.sin(), 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-angle.sin(), 0.0, angle.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
    pub fn rotation_z(angle: f32) -> Self {
        Self {
            data: [
                [angle.cos(), -angle.sin(), 0.0, 0.0],
                [angle.sin(), angle.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;
    fn mul(self, v: Vec4) -> Vec4 {
        Vec4::new(
            self.data[0][0] * v.x
                + self.data[0][1] * v.y
                + self.data[0][2] * v.z
                + self.data[0][3] * v.w,
            self.data[1][0] * v.x
                + self.data[1][1] * v.y
                + self.data[1][2] * v.z
                + self.data[1][3] * v.w,
            self.data[2][0] * v.x
                + self.data[2][1] * v.y
                + self.data[2][2] * v.z
                + self.data[2][3] * v.w,
            self.data[3][0] * v.x
                + self.data[3][1] * v.y
                + self.data[3][2] * v.z
                + self.data[3][3] * v.w,
        )
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Self;
    fn mul(self, other: Mat4) -> Self {
        let mut result = Mat4::zero();
        for r in 0..4 {
            for c in 0..4 {
                result.data[r][c] = self.data[r][0] * other.data[0][c]
                    + self.data[r][1] * other.data[1][c]
                    + self.data[r][2] * other.data[2][c]
                    + self.data[r][3] * other.data[3][c];
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.0001
    }

    #[test]
    fn identity_does_nothing() {
        let v = Vec4::new(1.0, 2.0, 3.0, 1.0);
        let result = Mat4::identity() * v;
        assert!(approx_eq(result.x, 1.0));
        assert!(approx_eq(result.y, 2.0));
        assert!(approx_eq(result.z, 3.0));
        assert!(approx_eq(result.w, 1.0));
    }

    #[test]
    fn translation_moves_point() {
        let v = Vec4::new(0.0, 0.0, 0.0, 1.0);
        let t = Mat4::translation(5.0, 3.0, 2.0);
        let result = t * v;
        assert!(approx_eq(result.x, 5.0));
        assert!(approx_eq(result.y, 3.0));
        assert!(approx_eq(result.z, 2.0));
    }

    #[test]
    fn translation_ignores_vector() {
        let v = Vec4::new(1.0, 1.0, 1.0, 0.0);
        let t = Mat4::translation(5.0, 3.0, 2.0);
        let result = t * v;
        assert!(approx_eq(result.x, 1.0));
        assert!(approx_eq(result.y, 1.0));
        assert!(approx_eq(result.z, 1.0));
    }

    #[test]
    fn scale_multiplies() {
        let v = Vec4::new(1.0, 2.0, 3.0, 1.0);
        let s = Mat4::scale(2.0, 3.0, 4.0);
        let result = s * v;
        assert!(approx_eq(result.x, 2.0));
        assert!(approx_eq(result.y, 6.0));
        assert!(approx_eq(result.z, 12.0));
    }

    #[test]
    fn rotation_z_90_degrees() {
        let v = Vec4::new(1.0, 0.0, 0.0, 1.0);
        let r = Mat4::rotation_z(PI / 2.0);
        let result = r * v;
        assert!(approx_eq(result.x, 0.0));
        assert!(approx_eq(result.y, 1.0));
        assert!(approx_eq(result.z, 0.0));
    }

    #[test]
    fn rotation_y_90_degrees() {
        let v = Vec4::new(1.0, 0.0, 0.0, 1.0);
        let r = Mat4::rotation_y(PI / 2.0);
        let result = r * v;
        assert!(approx_eq(result.x, 0.0));
        assert!(approx_eq(result.y, 0.0));
        assert!(approx_eq(result.z, -1.0));
    }

    #[test]
    fn rotation_x_90_degrees() {
        let v = Vec4::new(0.0, 1.0, 0.0, 1.0);
        let r = Mat4::rotation_x(PI / 2.0);
        let result = r * v;
        assert!(approx_eq(result.x, 0.0));
        assert!(approx_eq(result.y, 0.0));
        assert!(approx_eq(result.z, 1.0));
    }

    #[test]
    fn matrix_multiplication_combines_transforms() {
        let t = Mat4::translation(1.0, 0.0, 0.0);
        let s = Mat4::scale(2.0, 2.0, 2.0);
        let combined = t * s;
        let v = Vec4::new(1.0, 0.0, 0.0, 1.0);
        let result = combined * v;
        assert!(approx_eq(result.x, 3.0));
    }
}
