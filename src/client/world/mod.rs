pub mod chunk;

mod block;

use {
    std::collections::HashMap,
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
    pub world: InnerWorld<ClientChunk>,

    ind_buffer: Buffer,

    pub pipeline: Pipeline,
    pub ch_layout: Layout,

    pub atlas: Atlas,
}

const RDIST: usize = 8;

impl ClientWorld {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, renderer: &Renderer, player: &Player) -> Result<Self, super::err::ClientInitError> {
        let atlas = Atlas::new(device, queue)?;

        let ind_buffer = Buffer::new_quad_index_u32(device, CH_VOL as u32, "World Index Buffer");

        let ch_layout = Layout::uniform_layout(
            device,
            "Chunk Layout"
        );

        let world = InnerWorld::new(usize::pow(RDIST, 3));

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

        Ok(Self { world, ind_buffer, pipeline, ch_layout, atlas })
    }

    pub fn ch_exists(&self, pos: ChPos) -> bool {
        if let Some(_) = self.world.chunk_map.get(&pos) {
            true
        } else {
            false
        }
    }
}

impl crate::client::renderer::Render for ClientWorld {
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.pipeline.set(render_pass);
        self.atlas.set(0, render_pass);

        render_pass.set_index_buffer(self.ind_buffer.0.slice(..), wgpu::IndexFormat::Uint32);
        for (_, chunk) in self.world.chunk_map.iter() {
            chunk.pos_u.set(2, render_pass);

            render_pass.set_vertex_buffer(0, chunk.vb.0.slice(..));
            render_pass.draw_indexed(0..chunk.ind_count, 0, 0..1);
        }
    }
}