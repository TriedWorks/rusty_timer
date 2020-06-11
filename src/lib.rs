mod stopwatch;

// public exports
pub use stopwatch::*;


// TESTS


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
