//! Handles timer

use crate::prelude::*;

/// A timer
#[derive(Default, Clone)]
pub struct Timer {
    /// The amount duration passed
    pub current: Duration,
    /// The duration the timer finishes
    pub max: Duration,
}
impl Timer {
    /// Create a new timer with `max` duration and with `current` zero
    pub fn new(max: Duration) -> Timer {
        Timer { current: Duration::ZERO, max }
    }

    /// Create a new timer with `max` duration and with `current` at `max`
    pub fn new_ended(max: Duration) -> Timer {
        Timer { current: max, max }
    }

    /// Set `current` to zero
    pub fn reset(&mut self) {
        self.current = Duration::ZERO;
    }

    /// Check if `current` equals or more than `max`
    pub fn finished(&self) -> bool {
        self.current >= self.max
    }
}

/// Increment all timers by delta time
pub fn timer_system(delta: Duration, entities: &Entities, timers: &mut Components<Timer>) {
    for id in entities.iter() {
        let Ok(t) = timers.get_mut(entities, id) else { continue };
        t.current += delta;
    }
}
