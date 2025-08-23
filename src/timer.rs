use crate::prelude::*;

#[derive(Default, Clone)]
pub struct Timer {
    pub current: Duration,
    pub max: Duration,
}
impl Timer {
    pub fn new(max: Duration) -> Timer {
        Timer { current: Duration::ZERO, max }
    }

    pub fn new_ended(max: Duration) -> Timer {
        Timer { current: max, max }
    }

    pub fn reset(&mut self) {
        self.current = Duration::ZERO;
    }

    pub fn finished(&self) -> bool {
        self.current >= self.max
    }
}

pub fn timer_system(delta: Duration, entities: &Entities, timers: &mut Components<Timer>) {
    for id in entities.iter() {
        let Ok(t) = timers.get_mut(entities, id) else { continue };
        t.current += delta;
    }
}
