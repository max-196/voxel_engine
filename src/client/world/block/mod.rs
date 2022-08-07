pub mod atlas;
mod vertex;

pub use vertex::{BVertex, FaceVertices};

use atlas::AtlasCoords;

use crate::common::world::{
    Face::{self, *},
    Block::{self, *},
};

impl ClientBlock for Block {
    fn get_vertices(&self, face: Face, pos: [i32; 3]) -> FaceVertices {
        match self {
            Air => panic!("Tried to mesh air"),
            Grass => FaceVertices::get_face(face, pos, self.get_tex(face)),
        }
    }

    fn get_tex(&self, face: Face) -> AtlasCoords {
        AtlasCoords::new(
            match self {
                Air => panic!("Tried to texture air"),
                Grass => match face {
                    Top => 1,
                    Bottom => 0,
                    _ => 2,
                }
            }
        )
    }
}

pub trait ClientBlock {
    fn get_vertices(&self, face: Face, add: [i32; 3]) -> FaceVertices;
    fn get_tex(&self, face: Face) -> AtlasCoords;
}