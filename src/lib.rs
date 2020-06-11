mod stopwatch;
mod timer;
// public exports
pub use stopwatch::*;
pub use timer::*;

// TESTS


#[cfg(test)]
mod tests {
    use crate::{Stopwatch, Timer};
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
        stopwatch.restart();
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
        sys_stopwatch.restart();
        sleep(Duration::new(1, 0));
        let time_5 = sys_stopwatch.elapsed(); // ~ 1.0

        println!("time1: {:?}", time_1);
        println!("time2: {:?}", time_2);
        println!("time3: {:?}", time_3);
        println!("time4: {:?}", time_4);
        println!("time5: {:?}", time_5);
        println!("--- *** ---");
    }

    #[test]
    fn test_timer() {
        let mut timer = Timer::new();
        let mut x = 0;
        loop {
            timer.set_delta_time();
            println!("delta_time: {:?}", timer.delta_time());

            x += 1;
            if x >= 1000 {
                break
            }
        }
    }

    #[test]
    fn test_fixed_update() {
        let mut timer = Timer::new();
        timer.set_fixed_interval(Duration::from_secs_f32(1.0 / 120.0));
        let mut fixed_update_counter: u32 = 0;
        loop {
            timer.update_time_since_last_fixed();
            timer.set_delta_time();

            if timer.should_fixed_update() {
                fixed_update_counter += 1;
            }

            // println!("delta_time: {:?}", timer.delta_time());

            if timer.time_since_start() >= Duration::from_secs_f32(5.0) {
                println!("time: {:?}", timer.time_since_start());
                println!("fixed_updates expected: {:?}", timer.time_since_start() * 120);
                println!("fixed_updates amount: {}", fixed_update_counter);
                assert_eq!(fixed_update_counter, 120 * 5);
                break
            }
        }
    }
}
