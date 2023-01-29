use {
    crate::common::TimeCmpt,
    winit::{
        event::*,
        event_loop::ControlFlow,
        window::{Window, WindowId, CursorGrabMode},
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
        let dt = if let Some(stats) = &time.stats {
            format!("{} frames: {:.2} FPS; {:.2}s: {:.2} FPS", stats.max_frames, 1. / stats.f_avg_dt(), stats.max_time, 1. / stats.t_avg_dt())
        } else {
            format!("{:.2} FPS", 1./time.dt)
        };
        time.every(100, || self.window.set_title(&dt));
    }

    pub fn input(&mut self, event: &DeviceEvent, control_flow: &mut ControlFlow) {
        if let DeviceEvent::Key(KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Escape), state: ElementState::Pressed, ..}) = event {
            self.exit(control_flow);
        } else if let DeviceEvent::Key(KeyboardInput { virtual_keycode: Some(VirtualKeyCode::F1), state: ElementState::Pressed, ..}) = event {
            self.cursor_visible = !self.cursor_visible;

            const UNGRAB_MODE: CursorGrabMode = CursorGrabMode::None;
            const GRAB_MODE: CursorGrabMode = CursorGrabMode::Confined;
            if let Err(e) = self.window.set_cursor_grab(
                match self.cursor_visible {
                    true => UNGRAB_MODE,
                    false => GRAB_MODE,
                }) {log::error!("{}", e)}

            self.window.set_cursor_visible(self.cursor_visible);
        }
    }

    pub fn win_id(&self) -> WindowId {self.window.id()}

    pub fn req_redraw(&mut self) {self.window.request_redraw()}

    pub fn focused(&self) -> bool {self.focus}

    pub fn focus(&mut self, f: bool) {self.focus = f}

    pub fn exit(&self, control_flow: &mut ControlFlow) {*control_flow = ControlFlow::Exit}
}