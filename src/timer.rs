use std::time::Duration;
use crate::Stopwatch;

/// The Timer struct calculates delta time and if fixed updates should be done
/// to calculate the times, the crate's stopwatch is used
pub struct Timer {
    stop_watch: Stopwatch,
    time_since_start: Duration,
    current_time: Duration,
    previous_time: Duration,
    delta_time: Duration,
    fixed_interval: Duration,
    time_since_last_fixed_update: Duration,
    should_fixed: bool,
}

unsafe impl Send for Timer {}
unsafe impl Sync for Timer {}

impl Timer {
    /// Create a new timer object
    /// time since last fixed update is set to a high number on creation
    /// to get a fixed update on start
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the time since start of the timer
    pub fn time_since_start(&self) -> Duration {
        self.time_since_start
    }

    /// update the delta time and time since start
    /// place this on the beginning of the loop
    pub fn update_delta_time(&mut self) {
        self.current_time = self.stop_watch.elapsed();
        self.delta_time = self.current_time - self.previous_time;
        self.previous_time = self.current_time;
        self.time_since_start += self.delta_time;
    }

    /// update the time since last fixed update
    /// place this after update delta time
    /// this function is not required if no fixed update is used
    pub fn update_fixed_time(&mut self) {
        self.time_since_last_fixed_update += self.delta_time;
        if self.fixed_interval <= self.time_since_last_fixed_update  {
            self.time_since_last_fixed_update -= self.fixed_interval;
            self.should_fixed = true;
            return;
        }
        self.should_fixed = false;
    }

    /// returns a bool if a fixed update should be done
    /// this function is not required if no fixed update is used
    pub fn should_fixed_update(&self) -> bool {
        self.should_fixed
    }

    /// set the interval of a fixed update in seconds
    /// this function is not required if no fixed update is used
    pub fn set_fixed_interval(&mut self, fixed_interval: Duration) {
        self.fixed_interval = fixed_interval;
        self.time_since_last_fixed_update = fixed_interval;
    }

    /// public getter for the delta time
    pub fn delta_time(&self) -> Duration {
        self.delta_time
    }

    /// public getter for the delta fixed interval
    pub fn fixed_interval(&self) -> Duration {
        self.fixed_interval
    }

    /// public getter for the time since the last fixed update
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
            time_since_last_fixed_update: Duration::new(0, 0),
            should_fixed: false
        }
    }
}
