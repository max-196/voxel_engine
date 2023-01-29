pub struct Tick {
    pub ttime: f32,
    pub tcount: u64,
    dt: f32,
}

impl Tick {
    pub fn new(time: f32) -> Self {
        Self {
            ttime: time,
            tcount: 0,
            dt: 0.,
        }
    }

    pub fn update(&mut self, dt: f32) -> bool {
        self.dt += dt;
        if self.dt < self.ttime {
            false
        } else {
            self.dt = 0.;
            self.tcount += 1;
            true
        }
    }
}