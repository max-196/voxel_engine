mod camera;
mod controller;

use {
    camera::Camera,
    controller::Controller,
    super::TimeCmpt,
    crate::common::{
        math::{Angle, Vec3},
        world::pos::WorldPos,
    },
    winit::event::{VirtualKeyCode, ElementState, MouseScrollDelta}
};

const SMALL_FRAC_PI_2: f32 = std::f32::consts::FRAC_PI_2 - f32::EPSILON; // Subtracting epsilon prevents the camera from flipping over

pub struct Player {
    pub pos: WorldPos,
    yaw: Angle,
    pitch: Angle,
    pub camera: Camera,
    controller: Controller,
    velocity: Vec3<f32>,
    hrot: Angle,
    vrot: Angle,
    pub moved: bool,
}

impl Player {
    pub fn new(
        pos: WorldPos,
        yaw: Angle,
        pitch: Angle,
        width: u32,
        height: u32,
        device: &wgpu::Device,
    ) -> Self {
        let camera = Camera::new(width, height, Angle::from_deg(45.), 0.1, 500., device);
        let controller = Controller::new(1., 1.);

        Self {
            pos,
            yaw,
            pitch,
            camera,
            controller,
            velocity: Vec3::zero(),
            hrot: Angle::from_rad(0.),
            vrot: Angle::from_rad(0.),
            moved: false,
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
        if movement != [0., 0., 0.] {
            self.moved = true;
        }

        let (psin, pcos) = self.pitch.sin_cos();
        let (ysin, ycos) = self.yaw.sin_cos();
        let fw = Vec3::new(pcos * ycos, psin, pcos * ysin).norm();
        let right = Vec3::new(-ysin, 0.0, ycos).norm();
        self.velocity = (fw * movement[2]) + (right * movement[0]) + (Vec3::unit_y() * movement[1]);

        self.yaw += self.hrot * time.dt;
        self.pitch -= self.vrot * time.dt;
        if self.pitch.rad() >= SMALL_FRAC_PI_2 {self.pitch = Angle::from_rad(SMALL_FRAC_PI_2)}
        else if self.pitch.rad() <= -SMALL_FRAC_PI_2 {self.pitch = Angle::from_rad(-SMALL_FRAC_PI_2)}
        self.hrot = Angle::from_rad(0.);
        self.vrot = Angle::from_rad(0.);

        self.pos.add_x(self.velocity.x * time.dt);
        self.pos.add_y(self.velocity.y * time.dt);
        self.pos.add_z(self.velocity.z * time.dt);

        self.camera.update(self.pos, self.yaw, self.pitch, queue);
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.camera.resize(size.width, size.height);
    }
}