use winapi::um::profileapi::QueryPerformanceCounter;
use winapi::shared::ntdef::LARGE_INTEGER;
use std::mem::zeroed;

pub struct GameTimer {
    seconds_per_count: f64,
    delta_time: f64,
    base_time: i64,
    paused_time: i64,
    stop_time: i64,
    previous_time: i64,
    current_time: i64,
    stopped: bool
}

impl GameTimer {
    pub fn new() -> GameTimer {
        unsafe {
            let mut counts_per_second: LARGE_INTEGER = zeroed();
            QueryPerformanceCounter(&mut counts_per_second);
            let seconds_per_count = 1.0 / *counts_per_second.QuadPart() as f64;

            GameTimer {
                seconds_per_count: seconds_per_count,
                delta_time: -1.0,
                base_time: 0,
                paused_time: 0,
                stop_time: 0,
                previous_time: 0,
                current_time: 0,
                stopped: false
            }
        }
    }

    pub fn tick(&mut self) {
        if self.stopped {
            self.delta_time = 0.0;
            return;
        }

        unsafe {
            let mut current_time: LARGE_INTEGER = zeroed();
            QueryPerformanceCounter(&mut current_time);
            self.current_time = *current_time.QuadPart();
        }

        self.delta_time = (self.current_time - self.previous_time) as f64 * self.seconds_per_count;
        self.previous_time = self.current_time;
        
        if self.delta_time < 0.0 {
            self.delta_time = 0.0;
        }
    }

    pub fn reset(&mut self) {
        unsafe {
            let mut current_time: LARGE_INTEGER = zeroed();
            QueryPerformanceCounter(&mut current_time);

            self.base_time = *current_time.QuadPart();
            self.previous_time = *current_time.QuadPart();
            self.stop_time = *current_time.QuadPart();
            self.stopped = false;
        }
    }

    pub fn stop(&mut self) {
        if !self.stopped {
            unsafe {
                let mut current_time: LARGE_INTEGER = zeroed();
                QueryPerformanceCounter(&mut current_time);
                self.stop_time = *current_time.QuadPart();
                self.stopped = true;
            }
        }
    }

    pub fn start(&mut self) {
        if self.stopped {
            unsafe {
                let mut start_time: LARGE_INTEGER = zeroed();
                QueryPerformanceCounter(&mut start_time);

                self.paused_time += *start_time.QuadPart() - self.stop_time;
                self.previous_time = *start_time.QuadPart();
                self.stop_time = 0;
                self.stopped = false;
            }
        }
    }

    pub fn total_time(&self) -> f64 {
        if self.stopped {
            ((self.stop_time - self.paused_time) - self.base_time) as f64 * self.seconds_per_count
        }
        else {
            ((self.current_time - self.paused_time) - self.base_time) as f64 * self.seconds_per_count
        }
    }

    pub fn delta_time(&self) -> f64 {
        self.delta_time
    }
}