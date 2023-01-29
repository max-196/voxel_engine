use {
    super::{BVertex, ClientBlock},
    crate::{
        client::renderer::{bind_group::Layout, Buffer, Uniform},
        common::world::{pos::ChPos, *},
    }
};

pub struct ClientChunk {
    inner: InnerChunk,
    pub pos_u: Uniform<[i32; 3]>,
    pub vb: Buffer,
    pub ind_count: u32,
}

impl ClientChunk {
    pub fn new(inner: InnerChunk, device: &wgpu::Device, _layout: &Layout) -> Self {
        let pos = inner.pos;

        let pos_u = Uniform::new(device, [pos.0.x, pos.0.y, pos.0.z], "Client Position Uniform".to_owned());

        let vb = Buffer::new_vertex(device, &[], "Chunk Vertex Buffer");

        Self {
            inner,
            pos_u,
            vb,
            ind_count: 0,
        }
    }

    pub fn mesh(&mut self, device: &wgpu::Device) {
        let mut mesh: Vec<BVertex> = Vec::new();
        let mut indices = 0;
        for (ctr, b) in self.inner.blocks.iter().enumerate() {
            if let Block::Grass = b {
                let bi = ChInd(ctr);
                let bp = ChVec::from(bi);

                for face in FACES {
                    let fv = ChVec::from(face);
                    if (bp.0.x == 0 && fv.0.x == -1) || (bp.0.x == 31 && fv.0.x == 1) ||
                        (bp.0.y == 0 && fv.0.y == -1) || (bp.0.y == 31 && fv.0.y == 1) ||
                        (bp.0.z == 0 && fv.0.z == -1) || (bp.0.z == 31 && fv.0.z == 1) ||
                        self.inner.blocks[(bi + fv).0] == Block::Air {
                            let vertices = b.get_vertices(face, [bp.0.x, bp.0.y, bp.0.z]);
                            mesh.push(vertices.bl);
                            mesh.push(vertices.br);
                            mesh.push(vertices.tr);
                            mesh.push(vertices.tl);
                            indices += 6;
                        }
                }
            }
        }

        let vertex_buffer = Buffer::new_vertex(device, bytemuck::cast_slice(&mesh), "whatever");
        self.vb = vertex_buffer;
        self.ind_count = indices;
    }
}

impl Chunk for ClientChunk {}