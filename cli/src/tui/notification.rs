use std::time::{Duration, Instant};

pub struct Notification {
    message: String,
    created_at: Instant,
    duration: Duration,
}

impl Notification {
    pub fn new(message: &str, duration: Duration) -> Self {
        Self {
            message: String::from(message),
            created_at: Instant::now(),
            duration,
        }
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() >= self.duration
    }
}
