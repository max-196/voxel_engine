#[derive(Clone, Copy, Debug)]
pub struct Pnt3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl <T> Pnt3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl Pnt3<f32> {
    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(12);
        for i in [self.x, self.y, self.z] {
            for byte in i.to_be_bytes() {
                vec.push(byte);
            }
        }
        vec
    }

    pub fn from_be_bytes(src: &[u8]) -> Self {
        Self { x: f32::from_be_bytes([src[0], src[1], src[2], src[3]]),
            y: f32::from_be_bytes([src[4], src[5], src[6], src[7]]),
            z: f32::from_be_bytes([src[8], src[9], src[10], src[11]]) }
    }
}

impl Pnt3<i32> {
    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(12);
        for i in [self.x, self.y, self.z] {
            for byte in i.to_be_bytes() {
                vec.push(byte);
            }
        }
        vec
    }

    pub fn from_be_bytes(src: &[u8]) -> Self {
        Self { x: i32::from_be_bytes([src[0], src[1], src[2], src[3]]),
            y: i32::from_be_bytes([src[4], src[5], src[6], src[7]]),
            z: i32::from_be_bytes([src[8], src[9], src[10], src[11]]) }
    }
}