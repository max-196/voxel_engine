#[derive(Clone, Copy)]
pub struct Pnt3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl <T> Pnt3<T> {
    pub const fn new(x: T, y: T, z: T ) -> Self {
        Self { x, y, z }
    }
}