use std::time::{Duration, Instant, SystemTime};

/// A internal enum to manage stopwatch stopping
enum StopwatchState {
    Running,
    Paused,
}

/// The stopwatch uses the std::time Instant and Duration
pub struct Stopwatch {
    state: StopwatchState,
    duration: Duration,
    instant: Instant,
}

impl Stopwatch {
    pub fn new() -> Self {
        Self::default()
    }

    /// get the elapsed time since last restart
    pub fn elapsed(&self) -> Duration {
        match self.state {
            StopwatchState::Running => {
                self.duration + self.instant.elapsed()
            },
            StopwatchState::Paused => {
                self.duration
            },
        }

    }

    /// restart the stopwatch
    pub fn restart(&mut self) {
        self.duration = Duration::new(0, 0);
        self.instant = Instant::now();
        self.state = StopwatchState::Running;
    }

    /// pause the stopwatch
    pub fn pause(&mut self) {
        self.duration += self.instant.elapsed();
        self.state = StopwatchState::Paused;
    }

    /// unpause the stopwatch
    pub fn unpause(&mut self) {
        self.instant = Instant::now();
        self.state = StopwatchState::Running;
    }
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self {
            state: StopwatchState::Running,
            duration: Duration::new(0, 0),
            instant: Instant::now(),
        }
    }
}

/// This stopwatch is the same as the normal stopwatch
/// but instead of using std::Instance, it uses std::SystemTime
/// See normal Stopwatch for documentation
pub struct SystemStopwatch {
    state: StopwatchState,
    duration: Duration,
    instant: SystemTime,
}

impl SystemStopwatch {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn elapsed(&self) -> Duration {
        match self.state {
            StopwatchState::Running => {
                self.duration + self.instant.elapsed().unwrap()
            },
            StopwatchState::Paused => {
                self.duration
            },
        }

    }

    pub fn restart(&mut self) {
        self.duration = Duration::new(0, 0);
        self.instant = SystemTime::now();
        self.state = StopwatchState::Running;
    }

    pub fn pause(&mut self) {
        self.duration += self.instant.elapsed().unwrap();
        self.state = StopwatchState::Paused;
    }

    pub fn unpause(&mut self) {
        self.instant = SystemTime::now();
        self.state = StopwatchState::Running;
    }
}

impl Default for SystemStopwatch {
    fn default() -> Self {
        Self {
            state: StopwatchState::Running,
            duration: Duration::new(0, 0),
            instant: SystemTime::now(),
        }
    }
}