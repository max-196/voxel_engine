pub struct TimeCmpt {
    last: std::time::Instant,
    pub frame: u64,
    pub dt: f32,
}

impl TimeCmpt {
    pub fn new() -> Self {
        Self {
            last: std::time::Instant::now(),
            frame: 0,
            dt: 0.,
        }
    }

    /// Returns true every nth frame
    pub fn every(&self, n: u64) -> bool {
        self.frame % n == 0
    }

    pub fn update(&mut self) {
        self.dt = self.last.elapsed().as_secs_f32();
        self.last = std::time::Instant::now();
        self.frame += 1;
    }
}