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

    /// Executes ex every when frame value is a multiple of n (every n frames)
    pub fn every<F: FnMut()>(&self, n: u64, mut ex: F) {
        if self.frame % n == 0 {
            ex()
        }
    }

    /// Executes ex, timing it and printing the time elapsed every n frames, appending comment before the time value
    pub fn time_every<T, F: FnMut() -> T>(&self, n: u64, ex: F, comment: &str) -> T {
        let (ret, elapsed) = time(TimeUnit::MicroSecond, ex);
        self.every(n, || {println!("{}{:.3} ms", comment, elapsed / 1000.)});
        ret
    }

    pub fn update(&mut self) {
        self.dt = self.last.elapsed().as_secs_f32();
        self.last = std::time::Instant::now();
        self.frame += 1;
    }
}

pub struct Tick {
    pub tick_time: f32,
    pub tick: u64,
    dt: f32,
}

impl Tick {
    pub fn new(time: f32) -> Self {
        Self {
            tick_time: time,
            tick: 0,
            dt: 0.,
        }
    }

    pub fn update(&mut self, dt: f32) -> bool {
        self.dt += dt;
        if self.dt < self.tick_time {
            return false
        } else {
            self.dt = 0.;
            self.tick += 1;
            return true
        }
    }
}

pub enum TimeUnit {
    MicroSecond,
    MilliSecond,
    Second,
}

/// Executes ex and times it, returning the result of the ex and the time elapsed in t_unit
pub fn time<T, F: FnMut() -> T>(t_unit: TimeUnit, mut ex: F) -> (T, f64) {
    let start = std::time::Instant::now();
    let ret = ex();
    match t_unit {
        TimeUnit::MicroSecond => (ret, start.elapsed().as_micros() as f64),
        TimeUnit::MilliSecond => (ret, start.elapsed().as_millis() as f64),
        TimeUnit::Second => (ret, start.elapsed().as_secs_f64()),
    }
}