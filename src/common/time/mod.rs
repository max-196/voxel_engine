pub mod tick;
pub mod timing;
pub mod fstats;

use {
    std::time::Instant,
    fstats::FStats,
    timing::TimeUnit,
};

pub struct TimeCmpt {
    last: Instant,
    pub frame: u64,
    pub dt: f32,
    pub stats: Option<FStats>,
}

impl TimeCmpt {
    pub fn new(stats: Option<FStats>) -> Self {
        Self {
            last: Instant::now(),
            frame: 0,
            dt: 0.,
            stats,
        }
    }

    /// Executes ex every when frame value is a multiple of n (every n frames)
    pub fn every<F: FnMut()>(&self, n: u64, mut ex: F) {
        if self.frame % n == 0 {
            ex()
        }
    }

    /// Executes ex, timing it and printing the time elapsed every n frames, appending comment before the time value (For debugging)
    pub fn time_every<T, F: FnMut() -> T>(&self, n: u64, ex: F, comment: &str) -> T {
        let (ret, elapsed) = timing::time(TimeUnit::MicroSecond, ex);
        self.every(n, || {println!("{}{:.3} ms", comment, elapsed / 1000.)});
        ret
    }

    pub fn update(&mut self) {
        self.dt = self.last.elapsed().as_secs_f32();
        self.last = Instant::now();
        self.frame += 1;

        if let Some(fstats) = &mut self.stats {
            fstats.update(self.dt);
        }
    }
}