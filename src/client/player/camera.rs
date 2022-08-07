use {
    crate::{
        client::renderer::uniform::Uniform,
        common::{
            world::pos::WorldPos,
            math::{Mat4, Vec3, Rad}
        }
    },
    winit::{
        dpi::PhysicalPosition,
        event::MouseScrollDelta,
    }
};

pub struct Camera {
    aspect: f32,
    fovy: Rad,
    znear: f32,
    zfar: f32,
    uniform: Uniform<CamUniform>,
}

impl Camera {
    pub fn new(
        width: u32,
        height: u32,
        fovy: Rad,
        znear: f32,
        zfar: f32,
        device: &wgpu::Device,
    ) -> Self {
        let uniform = CamUniform::new();
        let uniform = Uniform::new(
            device,
            uniform,
            "Camera".to_string(),
        );
        println!("{}", fovy);

        Self {
            aspect: width as f32 / height as f32,
            fovy,
            znear,
            zfar,
            uniform,
        }
    }

    pub fn mouse_wheel(&mut self, delta: &MouseScrollDelta) {
        const LINE_MULTIPLIER: f32 = 1./15.;
        const PIXEL_MULTIPLIER: f64 = 1./30.;
        match delta {
            MouseScrollDelta::LineDelta(_, y) => {
                self.fovy += (y * LINE_MULTIPLIER).to_radians();
                println!("Changed FOV to {}", self.fovy.to_degrees());
            }
            MouseScrollDelta::PixelDelta(PhysicalPosition { y, ..}) => {
                self.fovy += (y / PIXEL_MULTIPLIER).to_radians() as f32;
                println!("Received pixel delta (scroll) {}", y);
            }
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn update(&mut self, pos: WorldPos, yaw: Rad, pitch: Rad, queue: &wgpu::Queue) {
        let cam_m = Mat4::look_to_rh(
            pos.inside,
            Vec3::new(pitch.cos() * yaw.cos(), pitch.sin(), pitch.cos() * yaw.sin()),
            Vec3::unit_y(),
        );

        let proj_m = Mat4::perspective(self.fovy, self.aspect, self.znear, self.zfar);

        self.uniform.data.update(cam_m, proj_m, pos);
        self.uniform.update(queue);
    }

    pub fn set_bg<'a>(&'a self, index: u32, render_pass: &mut wgpu::RenderPass<'a>) {
        self.uniform.set(index, render_pass);
    }

    pub fn get_uniform_bgl(&self) -> &wgpu::BindGroupLayout {
        &self.uniform.bg.layout.as_ref().unwrap().0
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
struct CamUniform {
    view_proj: [[f32; 4]; 4],
    chunk_pos: [i32; 3],
    padding: i32,
}

impl CamUniform {
    fn new() -> Self {
        Self {
            view_proj: Mat4::identity().into(),
            chunk_pos: [0; 3],
            padding: 0,
        }
    }

    fn update(&mut self, cam_matrix: Mat4<f32>, projection_matrix: Mat4<f32>, pos: WorldPos) {
        self.view_proj = (projection_matrix * cam_matrix).into();
        self.chunk_pos = [pos.chunk.0.x, pos.chunk.0.y, pos.chunk.0.z];
    }
}