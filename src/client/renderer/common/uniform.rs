use super::{
    buffer::Buffer,
    bind_group::BindGroup,
};

pub struct Uniform<T> {
    buf: Buffer,
    pub bg: BindGroup,
    pub data: T,
}

impl <T: bytemuck::Pod> Uniform<T> {
    pub fn new(
        device: &wgpu::Device,
        data: T,
        label: String,
    ) -> Self {

        let buf = Buffer::new_uniform(
            device,
            bytemuck::cast_slice(&[data]),
            &(label.clone() + " Uniform Buffer"),
        );

        let bg = BindGroup::uniform_bg(
            device,
            &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buf.0.as_entire_binding(),
            }],
            label,
        );

        Self { buf, bg, data }
    }

    pub fn update(&self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.buf.0, 0, bytemuck::cast_slice(&[self.data]));
    }

    pub fn set<'a>(&'a self, index: u32, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(index, &self.bg.group, &[]);
    }
}