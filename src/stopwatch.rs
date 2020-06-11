use std::time::{Duration, Instant, SystemTime};

enum StopwatchState {
    Running,
    Paused,
}

pub struct Stopwatch {
    state: StopwatchState,
    duration: Duration,
    instant: Instant,
}

impl Stopwatch {
    pub fn new() -> Self {
        Self::default()
    }

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

    pub fn start(&mut self) {
        self.duration = Duration::new(0, 0);
        self.instant = Instant::now();
        self.state = StopwatchState::Running;
    }

    pub fn pause(&mut self) {
        self.duration += self.instant.elapsed();
        self.state = StopwatchState::Paused;
    }

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

    pub fn start(&mut self) {
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