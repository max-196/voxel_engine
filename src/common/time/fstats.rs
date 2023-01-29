use std::collections::VecDeque;

pub struct FStats {
    framebound: FrameVec,
    pub max_frames: usize,
    timebound: FrameVec,
    pub max_time: f64,
}

impl FStats {
    pub fn new(max_frames: usize, max_time: f64) -> Self {
        Self {
            framebound: FrameVec::new(max_frames),
            max_frames,
            timebound: FrameVec::new((max_time * 60.) as usize),
            max_time,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.framebound.add(dt);
        if self.framebound.vec.len() >= self.max_frames {
            self.framebound.rm();
        }

        self.timebound.add(dt);
        while self.timebound.total >= self.max_time {
            self.timebound.rm();
        }
    }

    pub fn f_avg_dt(&self) -> f64 {
        self.framebound.avg_dt()
    }

    pub fn t_avg_dt(&self) -> f64 {
        self.timebound.avg_dt()
    }
}

struct FrameVec {
    vec: VecDeque<f32>,
    total: f64,
}

impl FrameVec {
    fn new(cap: usize) -> Self {
        Self {
            vec: VecDeque::with_capacity(cap),
            total: 0.,
        }
    }

    fn add(&mut self, dt: f32) {
        self.vec.push_back(dt);
        self.total += dt as f64;
    }

    fn rm(&mut self) {
        let dt = self.vec.pop_front();
        if let Some(dt) = dt {
            self.total -= dt as f64;
        } else {
            log::warn!("Tried removing value from frame stats, but the VecDeque is empty");
        }
    }

    pub fn avg_dt(&self) -> f64 {
        self.total / (self.vec.len() as f64)
    }

    // pub fn avg_fps(&self) -> f64 {
    //     1. / (self.total / (self.vec.len() as f64))
    // }

    // pub fn avg_dt_fps(&self) -> (f64, f64) {
    //     let dt = self.avg_dt();
    //     (dt, 1./dt)
    // }
}