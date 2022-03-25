use std::ops::Sub;
use crate::vector3d::Vector3d;

pub struct Matrix44d {
    pub m11: f32,
    pub m12: f32,
    pub m13: f32,
    pub m14: f32,
    pub m21: f32,
    pub m22: f32,
    pub m23: f32,
    pub m24: f32,
    pub m31: f32,
    pub m32: f32,
    pub m33: f32,
    pub m34: f32,
    pub m41: f32,
    pub m42: f32,
    pub m43: f32,
    pub m44: f32,
}

impl Matrix44d {

    pub fn new(m11: f32, m12: f32, m13: f32, m14: f32, m21: f32, m22: f32, m23: f32, m24: f32, m31: f32, m32: f32, m33: f32, m34: f32, m41: f32, m42: f32, m43: f32, m44: f32) -> Matrix44d {
        Matrix44d {
            m11: m11, m12: m12, m13: m13, m14: m14,
            m21: m21, m22: m22, m23: m23, m24: m24,
            m31: m31, m32: m32, m33: m33, m34: m34,
            m41: m41, m42: m42, m43: m43, m44: m44,
        }
    }


    pub fn translate(x: f32, y: f32, z: f32) -> Matrix44d {
        Matrix44d {
            m11: 1.0, m12: 0.0, m13: 0.0, m14: 0.0,
            m21: 0.0, m22: 1.0, m23: 0.0, m24: 0.0,
            m31: 0.0, m32: 0.0, m33: 1.0, m34: 0.0,
            m41: x, m42: y, m43: z, m44: 1.0,
        }
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Matrix44d {
        Matrix44d {
            m11: x, m12: 0.0, m13: 0.0, m14: 0.0,
            m21: 0.0, m22: y, m23: 0.0, m24: 0.0,
            m31: 0.0, m32: 0.0, m33: z, m34: 0.0,
            m41: 0.0, m42: 0.0, m43: 0.0, m44: 1.0,
        }
    }

    pub fn rotate(roll: f32, yaw: f32, pitch: f32) -> Matrix44d {
        let (sr, cr) = roll.sin_cos();
        let (sy, cy) = yaw.sin_cos();
        let (sp, cp) = pitch.sin_cos();
        Matrix44d {
            m11: cy * cp, m12: cy * sp * sr - sy * cr, m13: cy * sp * cr + sy * sr, m14: 0.0,
            m21: sy * cp, m22: sy * sp * sr + cy * cr, m23: sy * sp * cr - cy * sr, m24: 0.0,
            m31: -sp, m32: cp * sr, m33: cp * cr, m34: 0.0,
            m41: 0.0, m42: 0.0, m43: 0.0, m44: 1.0,
        }
    }

    pub fn identity(scale:f32) -> Matrix44d {
        Matrix44d {
            m11: scale, m12: 0.0, m13: 0.0, m14: 0.0,
            m21: 0.0, m22: scale, m23: 0.0, m24: 0.0,
            m31: 0.0, m32: 0.0, m33: scale, m34: 0.0,
            m41: 0.0, m42: 0.0, m43: 0.0, m44: scale,
        }
    }

    pub fn look_at(eye: Vector3d, center: Vector3d, up: Vector3d) -> Matrix44d {
        let f = (center.sub(eye)).normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(f);
        Matrix44d {
            m11: s.x, m12: u.x, m13: -f.x, m14: 0.0,
            m21: s.y, m22: u.y, m23: -f.y, m24: 0.0,
            m31: s.z, m32: u.z, m33: -f.z, m34: 0.0,
            m41: -s.dot(eye), m42: -u.dot(eye), m43: f.dot(eye), m44: 1.0,
        }
    }

    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Matrix44d {
        let f = 1.0 / (fov * 0.5).tan();
        let nf = 1.0 / (near - far);
        Matrix44d {
            m11: f / aspect, m12: 0.0, m13: 0.0, m14: 0.0,
            m21: 0.0, m22: f, m23: 0.0, m24: 0.0,
            m31: 0.0, m32: 0.0, m33: (far + near) * nf, m34: -1.0,
            m41: 0.0, m42: 0.0, m43: 2.0 * far * near * nf, m44: 0.0,
        }
    }

    pub fn to_list(&self) -> [[f32;4];4] {
        [[self.m11, self.m12, self.m13, self.m14],
         [self.m21, self.m22, self.m23, self.m24],
         [self.m31, self.m32, self.m33, self.m34],
         [self.m41, self.m42, self.m43, self.m44]]
    }

}