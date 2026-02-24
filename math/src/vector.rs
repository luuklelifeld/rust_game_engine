use crate::matrix::{Matrix2, Matrix3, Matrix4};

#[derive(Debug, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, PartialEq)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector2 {
    #[inline]
    pub const fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }
}

impl Vector3 {
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }
}

impl Vector4 {
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
        Vector4 { x, y, z, w }
    }
}

impl std::string::ToString for Vector2 {
    fn to_string(&self) -> String {
        format!("[{}, {}]", self.x, self.y)
    }
}

impl std::string::ToString for Vector3 {
    fn to_string(&self) -> String {
        format!("[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl std::string::ToString for Vector4 {
    fn to_string(&self) -> String {
        format!("[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
    }
}

impl std::ops::Mul<Matrix2> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: Matrix2) -> Self::Output {
        Vector2 {
            x: (rhs.x.x * self.x) + (rhs.x.y * self.y), // ax + by
            y: (rhs.y.x * self.x) + (rhs.y.y * self.y), // cx + dy
        }
    }
}

impl std::ops::Mul<Matrix3> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Matrix3) -> Self::Output {
        Vector3 {
            x: (rhs.x.x * self.x) + (rhs.x.y * self.y) + (rhs.x.z * self.z), // ax + by + cz
            y: (rhs.y.x * self.x) + (rhs.y.y * self.y) + (rhs.y.z * self.z), // dx + ey + fz
            z: (rhs.z.x * self.x) + (rhs.z.y * self.y) + (rhs.z.z * self.z), // gx + hy + iz
        }
    }
}

impl std::ops::Mul<Matrix4> for Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: Matrix4) -> Self::Output {
        Vector4 {
            x: (rhs.x.x * self.x) + (rhs.x.y * self.y) + (rhs.x.z * self.z) + (rhs.x.w * self.w),
            y: (rhs.y.x * self.x) + (rhs.y.y * self.y) + (rhs.y.z * self.z) + (rhs.y.w * self.w),
            z: (rhs.z.x * self.x) + (rhs.z.y * self.y) + (rhs.z.z * self.z) + (rhs.z.w * self.w),
            w: (rhs.w.x * self.x) + (rhs.w.y * self.y) + (rhs.w.z * self.z) + (rhs.w.w * self.w),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mat2mult() {
        let vector2 = Vector2::new(3.0, 4.0);
        let matrix2 = Matrix2::new(1.0, 3.0, 2.0, 1.0);
        let result = vector2 * matrix2;
        println!("{}", result.to_string());
        assert_eq!(result, Vector2::new(15.0, 10.0));
    }

    #[test]
    fn mat3mult() {
        let vector3 = Vector3::new(1.0, 1.0, 1.0);
        let matrix3 = Matrix3::new(2.0, 1.0, 0.0, -1.0, 3.0, 0.0, 0.0, 0.0, 4.0);
        let result = vector3 * matrix3;
        assert_eq!(result, Vector3::new(3.0, 2.0, 4.0))
    }

    #[test]
    fn mat4mult() {
        let vector4 = Vector4::new(2.0, 3.0, 1.0, 4.0);
        let matrix4 = Matrix4::new(
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 10.0, 11.0, 12.0,
            13.0, 14.0, 15.0, 16.0,
        );
        let result = vector4 * matrix4;
        assert_eq!(result, Vector4::new(27.0, 67.0, 107.0, 147.0))
    }
}
