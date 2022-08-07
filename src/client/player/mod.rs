mod camera;
mod controller;

use {
    camera::Camera,
    controller::Controller,
    super::TimeCmpt,
    crate::common::{
        math::{Rad, Vec3},
        world::pos::WorldPos,
    },
    winit::event::{VirtualKeyCode, ElementState, MouseScrollDelta}
};

const SMALL_FRAC_PI_2: f32 = std::f32::consts::FRAC_PI_2 - f32::EPSILON; // Subtracting epsilon prevents the camera from flipping over

pub struct Player {
    pub pos: WorldPos,
    yaw: Rad,
    pitch: Rad,
    pub camera: Camera,
    controller: Controller,
    velocity: Vec3<f32>,
    hrot: Rad,
    vrot: Rad,
}

impl Player {
    pub fn new(
        pos: WorldPos,
        yaw: Rad,
        pitch: Rad,
        width: u32,
        height: u32,
        device: &wgpu::Device,
    ) -> Self {
        let camera = Camera::new(width, height, 45_f32.to_radians(), 0.1, 500., device);
        let controller = Controller::new(1., 1.);

        Self {
            pos,
            yaw,
            pitch,
            camera,
            controller,
            velocity: Vec3::zero(),
            hrot: 0.,
            vrot: 0.,
        }
    }

    pub fn keyboard_input(&mut self, key: VirtualKeyCode, state: ElementState) {
        self.controller.keyboard_input(key, state);
    }

    pub fn mouse_movement(&mut self, dx: f64, dy: f64) {
        self.controller.mouse_movement(dx, dy);
        self.hrot = self.controller.hrot;
        self.vrot = self.controller.vrot;
    }

    pub fn mouse_wheel(&mut self, delta: &MouseScrollDelta) {
        self.camera.mouse_wheel(delta);
    }

    pub fn update(&mut self, queue: &wgpu::Queue, time: &TimeCmpt) {
        let movement = self.controller.get_movement();

        let fw = Vec3::new(self.pitch.cos() * self.yaw.cos(), self.pitch.sin(), self.pitch.cos() * self.yaw.sin()).norm();
        let right = Vec3::new(-self.yaw.sin(), 0.0, self.yaw.cos()).norm();
        self.velocity = (fw * movement[2]) + (right * movement[0]) + (Vec3::unit_y() * movement[1]);

        self.yaw += self.hrot * time.dt;
        self.pitch -= self.vrot * time.dt;
        if self.pitch >= SMALL_FRAC_PI_2 {self.pitch = SMALL_FRAC_PI_2}
        else if self.pitch <= -SMALL_FRAC_PI_2 {self.pitch = -SMALL_FRAC_PI_2}
        self.hrot = 0.;
        self.vrot = 0.;

        self.pos.add_x(self.velocity.x * time.dt);
        self.pos.add_y(self.velocity.y * time.dt);
        self.pos.add_z(self.velocity.z * time.dt);

        self.camera.update(self.pos, self.yaw, self.pitch, queue);

        if time.every(500) {
            println!("{} {} {}", self.pos.inside.x, self.pos.inside.y, self.pos.inside.z);
        }
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.camera.resize(size.width, size.height);
    }
}