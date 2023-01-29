pub mod err;
pub mod net;
pub mod world;

mod renderer;
mod player;
mod window;
mod time;

mod common;

use {
    common::*,
    crate::common::{
        world::pos::{WorldPos, ChPos},
        math::{Pnt3, Angle},
    },
    winit::{
        event::{DeviceEvent, KeyboardInput, WindowEvent},
        window::WindowId,
        event_loop::ControlFlow,
    },
};

pub struct Client {
    pub renderer: Renderer,
    pub player: Player,
    networking: Net,
    world: ClientWorld,
    window: WindowCmpt,
    time: TimeCmpt,
    net_tick: Tick,
}

impl Client {
    pub fn new(window: winit::window::Window, server_address: std::net::SocketAddr) -> Result<Self, self::err::ClientInitError> {
        let renderer = Renderer::new(&window)?;
        let player = Player::new(WorldPos::new(ChPos::new(Pnt3::new(-1, 0, 0)), Pnt3::new(31., 0., 0.)), Angle::from_rad(0.), Angle::from_rad(0.), renderer.config.width, renderer.config.height, &renderer.device);

        let world = ClientWorld::new(&renderer.device, &renderer.queue, &renderer, &player)?;

        let networking = Net::new(server_address)?;

        let window = WindowCmpt::new(window);

        let time = TimeCmpt::new();

        let net_tick = Tick::new(0.05);

        Ok(Self {
            renderer,
            player,
            networking,
            world,
            window,
            time,
            net_tick,
        })
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.renderer.render(&self.player, &self.world)
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.resize(size);
        self.player.resize(size);
    }

    pub fn input(&mut self, event: &winit::event::DeviceEvent, control_flow: &mut ControlFlow) {
        if self.window.focused() {

            self.window.input(event, control_flow);

            match event {
                DeviceEvent::Key(KeyboardInput { virtual_keycode: Some(key), state, ..}) => {
                    self.player.keyboard_input(*key, *state);
                }
                DeviceEvent::MouseMotion { delta } => {
                    self.player.mouse_movement(delta.0, delta.1);
                }
                DeviceEvent::MouseWheel { delta } => self.player.mouse_wheel(delta),
                _ => {}
            }
        }
    }

    pub fn update(&mut self, control_flow: &mut ControlFlow) {
        self.time.update();

        self.window.update(&self.time);
        self.player.update(&self.renderer.queue, &self.time);
        self.handle_server_messages();

        if self.net_tick.update(self.time.dt) {self.net_update()}

        match self.render() {
            Ok(_) => {}
            Err(wgpu::SurfaceError::Lost) => self.resize(self.renderer.size),
            Err(wgpu::SurfaceError::OutOfMemory) => self.window.exit(control_flow),
            Err(e) => log::error!("{}", e),
        }
    }

    fn net_update(&mut self) {
        if self.player.moved {
            self.networking.send(net::CMsg::position(&self.player.pos, self.networking.id));
            self.player.moved = false;
        }
    }

    pub fn same_id(&self, other: WindowId) -> bool {self.window.win_id() == other}

    pub fn req_redraw(&mut self) {self.window.req_redraw()}

    pub fn window_event(&mut self, event: &WindowEvent, control_flow: &mut ControlFlow) {
        match event {
            WindowEvent::Focused(f) => self.window.focus(*f),
            WindowEvent::CloseRequested => self.window.exit(control_flow),
            WindowEvent::Resized(physical_size) => {
                self.resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                self.resize(**new_inner_size);
            }
            _ => {}
        }
    }
}