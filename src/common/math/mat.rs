use {
    std::ops::Mul,
    super::{Vec4, Pnt3, Vec3}
};

#[derive(Clone, Copy)]
pub struct Mat4<T> {
    pub x: Vec4<T>,
    pub y: Vec4<T>,
    pub z: Vec4<T>,
    pub w: Vec4<T>,
}

impl <T> Mat4<T> {
    pub const fn new(c0: Vec4<T>, c1: Vec4<T>, c2: Vec4<T>, c3: Vec4<T>) -> Self {
        Self {
            x: c0,
            y: c1,
            z: c2,
            w: c3,
        }
    }
}

impl Mat4<f32> {
    pub fn look_to_rh(eye: Pnt3<f32>, dir: Vec3<f32>, up: Vec3<f32>) -> Self {
        let dir = dir.norm();
        let s = dir.cross(up).norm();
        let u = s.cross(dir);

        let eye: Vec3<f32> = eye.into();

        Self::new(
            Vec4::new(s.x, u.x, -dir.x, 0.),
            Vec4::new(s.y, u.y, -dir.y, 0.),
            Vec4::new(s.z, u.z, -dir.z, 0.),
            Vec4::new(-eye.dot(s), -eye.dot(u), eye.dot(dir), 1.),
        )
    }

    pub fn perspective(fovy: super::Angle, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1. / (fovy.rad() / 2.).tan();
        Self::new(
            Vec4::new(f / aspect, 0., 0., 0.),
            Vec4::new(0., f, 0., 0.),
            Vec4::new(0., 0., (far + near) / (near - far), -1.),
            Vec4::new(0., 0., (far * near) / (near - far), 0.),
            //Vec4::new(0., 0., (2*. far * near) / (near - far), 0.) for OpenGL coordinates
        )
    }

    pub fn identity() -> Self {
        Self::new(
            Vec4::new(1., 0., 0., 0.),
            Vec4::new(0., 1., 0., 0.),
            Vec4::new(0., 0., 1., 0.),
            Vec4::new(0., 0., 0., 1.),
        )
    }
}

impl Mul<Mat4<f32>> for Mat4<f32> {
    type Output = Self;
    fn mul(self, rhs: Mat4<f32>) -> Self::Output {
        Self::new(
            Vec4::new(
                Vec4::new(self.x.x, self.y.x, self.z.x, self.w.x).dot(rhs.x),
                Vec4::new(self.x.y, self.y.y, self.z.y, self.w.y).dot(rhs.x),
                Vec4::new(self.x.z, self.y.z, self.z.z, self.w.z).dot(rhs.x),
                Vec4::new(self.x.w, self.y.w, self.z.w, self.w.w).dot(rhs.x),
            ),
            Vec4::new(
                Vec4::new(self.x.x, self.y.x, self.z.x, self.w.x).dot(rhs.y),
                Vec4::new(self.x.y, self.y.y, self.z.y, self.w.y).dot(rhs.y),
                Vec4::new(self.x.z, self.y.z, self.z.z, self.w.z).dot(rhs.y),
                Vec4::new(self.x.w, self.y.w, self.z.w, self.w.w).dot(rhs.y),
            ),
            Vec4::new(
                Vec4::new(self.x.x, self.y.x, self.z.x, self.w.x).dot(rhs.z),
                Vec4::new(self.x.y, self.y.y, self.z.y, self.w.y).dot(rhs.z),
                Vec4::new(self.x.z, self.y.z, self.z.z, self.w.z).dot(rhs.z),
                Vec4::new(self.x.w, self.y.w, self.z.w, self.w.w).dot(rhs.z),
            ),
            Vec4::new(
                Vec4::new(self.x.x, self.y.x, self.z.x, self.w.x).dot(rhs.w),
                Vec4::new(self.x.y, self.y.y, self.z.y, self.w.y).dot(rhs.w),
                Vec4::new(self.x.z, self.y.z, self.z.z, self.w.z).dot(rhs.w),
                Vec4::new(self.x.w, self.y.w, self.z.w, self.w.w).dot(rhs.w),
            ),
        )
    }
}

impl <T> From<Mat4<T>> for [[T; 4]; 4] {
    fn from(mat: Mat4<T>) -> Self {
        [
            mat.x.into(),
            mat.y.into(),
            mat.z.into(),
            mat.w.into()
        ]
    }
}