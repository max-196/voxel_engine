use {
    super::CH_SIZE,
    crate::common::math::Pnt3,
};

#[derive(Clone, Copy, Debug)]
pub struct WorldPos {
    pub chunk: ChPos,
    pub inside: Pnt3<f32>,
}

impl WorldPos {
    pub const fn new(chunk: ChPos, inside: Pnt3<f32>) -> Self {
        Self { chunk, inside }
    }

    pub fn add_x(&mut self, x: f32) {
        self.inside.x += x;
        self.check_x();
    }

    fn check_x(&mut self) {
        if self.inside.x >= CH_SIZE as f32 {
            self.inside.x -= CH_SIZE as f32;
            self.chunk.0.x += 1;
        } else if self.inside.x < 0. {
            self.inside.x += CH_SIZE as f32;
            self.chunk.0.x -= 1;
        }
    }

    pub fn add_y(&mut self, y: f32) {
        self.inside.y += y;
        self.check_y();
    }

    fn check_y(&mut self) {
        if self.inside.y >= CH_SIZE as f32 {
            self.inside.y -= CH_SIZE as f32;
            self.chunk.0.y += 1;
        } else if self.inside.y < 0. {
            self.inside.y += CH_SIZE as f32;
            self.chunk.0.y -= 1;
        }
    }

    pub fn add_z(&mut self, z: f32) {
        self.inside.z += z;
        self.check_z();
    }

    fn check_z(&mut self) {
        if self.inside.z >= CH_SIZE as f32 {
            self.inside.z -= CH_SIZE as f32;
            self.chunk.0.z += 1;
        } else if self.inside.z < 0. {
            self.inside.z += CH_SIZE as f32;
            self.chunk.0.z -= 1;
        }
    }

    pub fn as_be_bytes(&self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(24);
        vec.extend_from_slice(&self.chunk.0.as_be_bytes());
        vec.extend_from_slice(&self.inside.as_be_bytes());
        vec
    }

    pub fn from_be_bytes(src: &[u8]) -> Self {
        Self {
            chunk: ChPos(Pnt3::<i32>::from_be_bytes(&src[0..12])),
            inside: Pnt3::<f32>::from_be_bytes(&src[12..24]),
        }
    }
}

impl Default for WorldPos {
    fn default() -> Self {
        Self { chunk: ChPos::new(Pnt3::new(0, 0, 0)), inside: Pnt3::new(0., 0., 0.) }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChPos (pub Pnt3<i32>);

impl ChPos {
    pub fn new(p: Pnt3<i32>) -> Self {
        Self(p)
    }
}