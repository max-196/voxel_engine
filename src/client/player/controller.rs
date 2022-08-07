use winit::event::{VirtualKeyCode, ElementState};

pub struct Controller {
    speed: f32,
    sens: f32,

    forward: bool,
    backward: bool,
    right: bool,
    left: bool,
    up: bool,
    down: bool,

    pub hrot: f32,
    pub vrot: f32,
}

impl Controller {
    pub fn new(speed: f32, sens: f32) -> Self {
        Self {
            speed,
            sens,

            forward:  false,
            backward: false,
            right:    false,
            left:     false,
            up:       false,
            down:     false,

            hrot: 0.0,
            vrot: 0.0,
        }
    }

    pub fn keyboard_input(&mut self, key: VirtualKeyCode, state: ElementState) {
        let mov = state == ElementState::Pressed;

        match key {
            VirtualKeyCode::W => {
                self.forward = mov;
            }
            VirtualKeyCode::S => {
                self.backward = mov;
            }
            VirtualKeyCode::D => {
                self.right = mov;
            }
            VirtualKeyCode::A => {
                self.left = mov;
            }
            VirtualKeyCode::Space => {
                self.up = mov;
            }
            VirtualKeyCode::LShift => {
                self.down = mov;
            }
            _ => {},
        }
    }

    pub fn get_movement(&self) -> [f32; 3] { // xyz
        [
            (self.right as i8 - self.left as i8) as f32 * self.speed,
            (self.up as i8 - self.down as i8) as f32 * self.speed,
            (self.forward as i8 - self.backward as i8) as f32 * self.speed,
        ]
    }

    pub fn mouse_movement(&mut self, dx: f64, dy: f64) {
        self.hrot = dx as f32 * self.sens;
        self.vrot = dy as f32 * self.sens;
    }
}