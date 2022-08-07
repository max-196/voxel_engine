use {
    super::time::TimeCmpt,
    winit::{
        event::*,
        event_loop::ControlFlow,
        window::{Window, WindowId},
    }
};

pub struct WindowCmpt {
    window: Window,
    focus: bool,
    cursor_visible: bool,
}

impl WindowCmpt {
    pub fn new(window: Window) -> Self {
        Self {
            window,
            focus: true,
            cursor_visible: true,
        }
    }

    pub fn update(&mut self, time: &TimeCmpt) {
        if time.every(100) {
            self.window.set_title(&(1. / time.dt).to_string())
        }
    }

    pub fn input(&mut self, event: &DeviceEvent, control_flow: &mut ControlFlow) {
        if let DeviceEvent::Key(KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Escape), state: ElementState::Pressed, ..}) = event {
            self.exit(control_flow);
        } else if let DeviceEvent::Key(KeyboardInput { virtual_keycode: Some(VirtualKeyCode::F1), state: ElementState::Pressed, ..}) = event {
            self.cursor_visible = !self.cursor_visible;
            if let Err(e) = self.window.set_cursor_grab(!self.cursor_visible) {log::error!("{}", e)}
            self.window.set_cursor_visible(self.cursor_visible);
        }
    }

    pub fn win_id(&self) -> WindowId {self.window.id()}

    pub fn req_redraw(&mut self) {self.window.request_redraw()}

    pub fn focused(&self) -> bool {self.focus}

    pub fn focus(&mut self, f: bool) {self.focus = f}

    pub fn exit(&self, control_flow: &mut ControlFlow) {*control_flow = ControlFlow::Exit}
}