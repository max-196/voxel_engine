use {
    crate::client::{renderer::Texture, err::ClientInitError},
    wgpu::RenderPass,
};

pub struct Atlas(pub Texture);

impl Atlas {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Result<Self, ClientInitError> {

        let texture = Texture::new(device, queue, "Atlas".to_owned(), "assets/sprites/atlas.png")?;

        Ok(Self(texture))
    }

    pub fn set<'a>(&'a self, index: u32, render_pass: &mut RenderPass<'a>) {
        render_pass.set_bind_group(index, &self.0.bind_group.group, &[]);
    }
}

const WIDTH: f32 = 8.;
const HEIGHT: f32 = 8.;

const UWIDTH: usize = 8;
const UHEIGHT: usize = 8;

pub struct AtlasCoords {
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
    pub top: f32,
}

impl AtlasCoords {
    pub fn new(i: usize) -> Self {
        let (l, t) = (i % UWIDTH, i / UWIDTH);
        let (l, b, r, t) = (l as f32, (t + 1) as f32, (l + 1) as f32, t as f32);
        Self {
            left:   1. / WIDTH * l,
            bottom: 1. / HEIGHT * b,
            right:  1. / WIDTH * r,
            top:    1. / HEIGHT * t,
        }
    }
}