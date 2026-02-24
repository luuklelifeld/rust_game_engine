use crate::vector::{Vector2, Vector3, Vector4};

pub struct Matrix2 {
    pub x: Vector2,
    pub y: Vector2,
}

pub struct Matrix3 {
    pub x: Vector3,
    pub y: Vector3,
    pub z: Vector3,
}

pub struct Matrix4 {
    pub x: Vector4,
    pub y: Vector4,
    pub z: Vector4,
    pub w: Vector4,
}

impl Matrix2 {
    #[inline]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn new(
        a: f32, b: f32, 
        c: f32, d: f32
    ) -> Matrix2 {
        Matrix2 {
            x: Vector2::new(a, b),
            y: Vector2::new(c, d),
        }
    }
}

impl Matrix3 {
    #[inline]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn new(
        a: f32, b: f32, c: f32, 
        d: f32, e: f32, f: f32,
        g: f32, h: f32, i: f32
    ) -> Matrix3 {
        Matrix3 {
            x: Vector3::new(a, b, c),
            y: Vector3::new(d, e, f),
            z: Vector3::new(g, h, i)
        }
    }
}

impl Matrix4 {
    #[inline]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn new(
       a: f32, b: f32, c: f32, d: f32,
       e: f32, f: f32, g: f32, h: f32,
       i: f32, j: f32, k: f32, l: f32,
       m: f32, n: f32, o: f32, p: f32
    ) -> Matrix4 {
        Matrix4 {
            x: Vector4::new(a, b, c, d),
            y: Vector4::new(e, f, g, h),
            z: Vector4::new(i, j, k, l),
            w: Vector4::new(m, n, o, p)
        }
    }
}
