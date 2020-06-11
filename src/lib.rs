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

#[cfg(test)]
mod tests {
    use super::Stopwatch;
    use std::time::Duration;
    use std::thread::sleep;
    use crate::SystemStopwatch;

    #[test]
    fn test_stopwatch() {
        println!("--- Testing normal stopwatch ---");
        let mut stopwatch = Stopwatch::new();
        let time_1 = stopwatch.elapsed(); // ~ 0
        sleep(Duration::new(2, 0));
        let time_2 = stopwatch.elapsed(); // ~ 2.0
        stopwatch.pause();
        sleep(Duration::new(1, 0));
        let time_3 = stopwatch.elapsed(); // ~ 2.0
        stopwatch.unpause();
        sleep(Duration::new(1, 0));
        let time_4 = stopwatch.elapsed(); // ~ 3.0
        stopwatch.start();
        sleep(Duration::new(1, 0));
        let time_5 = stopwatch.elapsed(); // ~ 1.0

        println!("time1: {:?}", time_1);
        println!("time2: {:?}", time_2);
        println!("time3: {:?}", time_3);
        println!("time4: {:?}", time_4);
        println!("time5: {:?}", time_5);
        println!("--- *** ---");
    }

    #[test]
    fn test_system_stopwatch() {
        println!("Testing system stopwatch");
        let mut sys_stopwatch = SystemStopwatch::new();
        let time_1 = sys_stopwatch.elapsed(); // ~ 0
        sleep(Duration::new(2, 0));
        let time_2 = sys_stopwatch.elapsed(); // ~ 2.0
        sys_stopwatch.pause();
        sleep(Duration::new(1, 0));
        let time_3 = sys_stopwatch.elapsed(); // ~ 2.0
        sys_stopwatch.unpause();
        sleep(Duration::new(1, 0));
        let time_4 = sys_stopwatch.elapsed(); // ~ 3.0
        sys_stopwatch.start();
        sleep(Duration::new(1, 0));
        let time_5 = sys_stopwatch.elapsed(); // ~ 1.0

        println!("time1: {:?}", time_1);
        println!("time2: {:?}", time_2);
        println!("time3: {:?}", time_3);
        println!("time4: {:?}", time_4);
        println!("time5: {:?}", time_5);
        println!("--- *** ---");
    }
}
