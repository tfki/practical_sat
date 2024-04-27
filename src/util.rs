use std::time::{Duration, SystemTime};

#[derive(Debug, Copy, Clone)]
pub struct Timer {
    start: SystemTime,
    duration: Duration,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Timer { start: SystemTime::now(), duration }
    }

    pub fn new_infinite() -> Self { Timer { start: SystemTime::now(), duration: Duration::MAX } }

    pub fn has_finished(&self) -> bool {
        SystemTime::now().duration_since(self.start).unwrap() > self.duration
    }
}
