use std::time::Duration;
use crate::Stopwatch;

pub struct Timer {
    stop_watch: Stopwatch,
    time_since_start: Duration,
    current_time: Duration,
    previous_time: Duration,
    delta_time: Duration,

    // how long a fixed update should take
    fixed_interval: Duration,
    time_since_last_fixed_update: Duration,
}

impl Timer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn time_since_start(&self) -> Duration {
        self.time_since_start
    }

    pub fn set_delta_time(&mut self) {
        self.current_time = self.stop_watch.elapsed();
        self.delta_time = self.current_time - self.previous_time;
        self.previous_time = self.current_time;
        self.time_since_start += self.delta_time;
    }

    pub fn update_time_since_last_fixed(&mut self) {
        self.time_since_last_fixed_update += self.delta_time;
    }

    pub fn delta_time(&self) -> Duration {
        self.delta_time
    }

    pub fn should_fixed_update(&mut self) -> bool {
        if self.fixed_interval <= self.time_since_last_fixed_update {
            self.time_since_last_fixed_update *= 0;
            true
        } else {
            false
        }
    }

    pub fn set_fixed_interval(&mut self, fixed_interval: Duration) {
        self.fixed_interval = fixed_interval;
    }

    pub fn fixed_interval(&self) -> Duration {
        self.fixed_interval
    }

    pub fn time_since_last_fixed(&self) -> Duration {
        self.time_since_last_fixed_update
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            stop_watch: Stopwatch::new(),
            time_since_start: Duration::new(0, 0),
            current_time: Duration::new(0, 0),
            previous_time: Duration::new(0, 0),
            delta_time: Duration::new(0, 0),
            fixed_interval: Duration::new(0, 0),
            time_since_last_fixed_update: Duration::new(1, 0)
        }
    }
}
