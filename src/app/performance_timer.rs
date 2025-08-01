use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct PerformanceTimer {
    durations: HashMap<String, Duration>,

    last_change: Instant,
    current_category: String,

    real: bool,
}

impl Drop for PerformanceTimer {
    fn drop(&mut self) {
        if !self.real {
            return;
        }
        self.set_category("idle");

        let mut total_duration = Duration::new(0, 0);

        for (_, duration) in &self.durations {
            total_duration += *duration;
        }

        for (category, duration) in &self.durations {
            let percentage = if total_duration.as_secs_f64() > 0.0 {
                (duration.as_secs_f64() / total_duration.as_secs_f64()) * 100.0
            } else {
                0.0
            };

            println!("{}: {:.2} seconds ({:.2}%)", category, duration.as_secs_f64(), percentage);
        }
    }
}


impl PerformanceTimer {
    pub fn new() -> Self {
        Self {
            durations: HashMap::new(),
            last_change: Instant::now(),
            current_category: "idle".to_string(),

            real: true,
        }
    }

    pub fn new_fake() -> Self {
        Self {
            durations: HashMap::new(),
            last_change: Instant::now(),
            current_category: "idle".to_string(),

            real: false,
        }
    }

    pub fn set_category(&mut self, category: &str) {
        let now = Instant::now();
        if let Some(duration) = self.durations.get_mut(&self.current_category) {
            *duration += now.duration_since(self.last_change);
        } else {
            self.durations.insert(self.current_category.clone(), now.duration_since(self.last_change));
        }

        self.current_category = category.to_string();
        self.last_change = Instant::now();
    }

    pub fn reset(&mut self) {
        self.durations.clear();
        self.last_change = Instant::now();
        self.current_category = "idle".to_string();
    }
}
