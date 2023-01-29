
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