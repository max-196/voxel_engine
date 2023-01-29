use wgpu::RenderPass;

use super::{shader::Shader, super::depth_texture::{self, DepthValue}};

pub struct Pipeline {
    _layout: wgpu::PipelineLayout,
    pub pipeline: wgpu::RenderPipeline,
}

impl Pipeline {
    pub fn new(
        v_shader: Shader,
        f_shader: Shader,
        device: &wgpu::Device,
        bg_layouts: &[&wgpu::BindGroupLayout],
        targets: &[Option<wgpu::ColorTargetState>],
        depth_value: DepthValue,
        vert_buffer_layout: wgpu::VertexBufferLayout,
    ) -> Self {

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: bg_layouts,
            push_constant_ranges: &[],
        });


        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module:      &v_shader.module,
                entry_point: v_shader.entry,
                buffers:     &[vert_buffer_layout],
            },
            fragment: Some(wgpu::FragmentState {
                module: &f_shader.module,
                entry_point: f_shader.entry,
                targets,
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: match depth_value {
                DepthValue::None => None,
                DepthValue::ReadOnly | DepthValue::ReadWrite => {
                    Some(wgpu::DepthStencilState {
                        format: depth_texture::DepthTexture::DEPTH_FORMAT,
                        depth_write_enabled: matches!(depth_value, DepthValue::ReadWrite),
                        depth_compare: wgpu::CompareFunction::Less,
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default(),
                    })
                },
            },
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self { _layout: layout, pipeline }
    }

    pub fn set<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
    }
}