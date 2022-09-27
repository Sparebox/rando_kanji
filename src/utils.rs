use std::time::{SystemTime, Duration};

use sfml::system::{Vector2f, Vector2i};

pub fn vector2f_to_vector2i(vector: Vector2f) -> Vector2i {
    Vector2i::new(vector.x as i32, vector.y as i32)
}

#[derive(Clone, Debug)]
pub struct Timer {
    time: SystemTime,
    duration: Duration,
}

impl Timer {
    pub fn new(secs: f32) -> Self {
        Self {
            time: SystemTime::now(),
            duration: Duration::from_secs_f32(secs),
        }
    }

    pub fn reset(&mut self) {
        self.time = SystemTime::now();
    }

    /// Check if the timer has expired and reset and return true if it has.
    /// Otherwise return false
    pub fn check(&mut self) -> bool {
        if self.time.elapsed().unwrap_or_default() > self.duration {
            self.reset();
            true
        } else {
            false
        }
    }

    pub fn set_duration(&mut self, secs: f32) {
        self.duration = Duration::from_secs_f32(secs);
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            time: SystemTime::now(),
            duration: Duration::default(),
        }
    }
}
