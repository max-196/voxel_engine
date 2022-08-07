pub mod chunk;

mod block;

use {
    block::{BVertex, ClientBlock},
    chunk::ClientChunk,
    block::atlas::Atlas,
    super::{
        Player,
        renderer::{Renderer, Shader, depth_texture, Pipeline, bind_group::Layout, Buffer}
    },
    crate::common::{
        world::{pos::ChPos, InnerWorld, CH_VOL},
        math::Pnt3
    }
};

pub struct ClientWorld {
    world: InnerWorld<ClientChunk>,

    ind_buffer: Buffer,

    pub pipeline: Pipeline,
    ch_layout: Layout,

    pub atlas: Atlas,
}

impl ClientWorld {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, renderer: &Renderer, player: &Player) -> Result<Self, super::err::ClientInitError> {
        let atlas = Atlas::new(device, queue)?;

        let ind_buffer = Buffer::new_quad_index_u32(device, CH_VOL as u32, "World Index Buffer");

        let ch_layout = Layout::uniform_layout(
            device,
            "Chunk Layout"
        );

        let mut chunk_list = Vec::with_capacity(8 * 2 * 8);

        for x in -4..4 {
            for y in -1..1 {
                for z in -4..4 {
                    let mut chunk = ClientChunk::new(device, &ch_layout, ChPos::new(Pnt3::new(x, y, z)));
                    chunk.mesh(device);
                    chunk_list.push(chunk);
                }
            }
        }

        let pipeline = Pipeline::new(
            Shader::new(&renderer.device, "Vertex Shader", "assets/shaders/vert.wgsl", "main")?,
            Shader::new(&renderer.device, "Fragment Shader", "assets/shaders/frag.wgsl", "main")?,
            &renderer.device,
            &[&atlas.0.bind_group.layout.as_ref().unwrap().0, player.camera.get_uniform_bgl(), &ch_layout.0],
            &[Some(wgpu::ColorTargetState {
                format: renderer.config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            depth_texture::DepthValue::ReadWrite,
            BVertex::desc(),
        );

        let world = InnerWorld::new(chunk_list);

        Ok(Self { world, ind_buffer, pipeline, ch_layout, atlas })
    }
}

impl crate::client::renderer::Render for ClientWorld {
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.pipeline.set(render_pass);
        self.atlas.set(0, render_pass);

        render_pass.set_index_buffer(self.ind_buffer.0.slice(..), wgpu::IndexFormat::Uint32);
        for (ctr, _) in self.world.chunk_list.iter().enumerate() {
            self.world.chunk_list[ctr].pos_u.set(2, render_pass);

            render_pass.set_vertex_buffer(0, self.world.chunk_list[ctr].vb.0.slice(..));
            render_pass.draw_indexed(0..self.world.chunk_list[ctr].ind_count, 0, 0..1);
        }
    }
}