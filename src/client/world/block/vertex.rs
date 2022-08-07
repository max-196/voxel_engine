use super::{Face::{self, *}, AtlasCoords};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl BVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
    wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<BVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }

    fn new(position: [f32; 3], tex_coords: [f32; 2]) -> Self {
        Self { position, tex_coords }
    }
}

pub struct FaceVertices {
    pub bl: BVertex, // Bottom Left
    pub br: BVertex, // Bottom Right
    pub tr: BVertex, // Top Right
    pub tl: BVertex, // Top Left
}

impl FaceVertices {
    pub fn get_face(face: Face, pos: [i32; 3], tex: AtlasCoords) -> Self {
        let x = (pos[0] as f32, (pos[0] + 1) as f32);
        let y = (pos[1] as f32, (pos[1] + 1) as f32);
        let z = (pos[2] as f32, (pos[2] + 1) as f32);

        let fin = match face {
            Front =>[
                [x.0, y.0, z.0],
                [x.0, y.0, z.1],
                [x.0, y.1, z.1],
                [x.0, y.1, z.0],
            ],
            Right => [
                [x.0, y.0, z.1],
                [x.1, y.0, z.1],
                [x.1, y.1, z.1],
                [x.0, y.1, z.1],
            ],
            Back => [
                [x.1, y.0, z.1],
                [x.1, y.0, z.0],
                [x.1, y.1, z.0],
                [x.1, y.1, z.1],
            ],
            Left => [
                [x.1, y.0, z.0],
                [x.0, y.0, z.0],
                [x.0, y.1, z.0],
                [x.1, y.1, z.0],
            ],
            Top => [
                [x.0, y.1, z.0],
                [x.0, y.1, z.1],
                [x.1, y.1, z.1],
                [x.1, y.1, z.0],
            ],
            Bottom => [
                [x.1, y.0, z.0],
                [x.1, y.0, z.1],
                [x.0, y.0, z.1],
                [x.0, y.0, z.0],
            ],
        };

        Self {
            bl: BVertex::new(fin[0], [tex.left, tex.bottom]),
            br: BVertex::new(fin[1], [tex.right, tex.bottom]),
            tr: BVertex::new(fin[2], [tex.right, tex.top]),
            tl: BVertex::new(fin[3], [tex.left, tex.top]),
        }
    }
}