//! Handles damage

use crate::prelude::*;

/// Damage event
///
/// This event is manually emitted by any system that needs to declare some
/// damage is being done to an entity.
pub struct Damage {
    /// The [`Entity`] that is being damaged
    pub target: Entity,
    /// The damage amount
    pub amount: i32,
}

/// Kill event
///
/// This event is emitted when an entity with the [`Health`] component reaches
/// or less than 0.
pub struct Kill {
    /// The [`Entity`] that have been killed
    pub target: Entity,
}

/// This system handle emits bell notification when there are any [`Damage`]
/// event in the buffer.
/// If an entity's health reaches or less than zero, emit the [`Kill`] event.
/// If an entity has `damaged_timer`, it will be reset. This can be used to
/// implemented animations.
pub fn damage_system(stdout: &mut Stdout, damage_events: &Events<Damage>, kill_events: &mut Events<Kill>, entities: &Entities, hps: &mut Components<Health>, damaged_timer: &mut Components<Timer>) {
    if !damage_events.is_empty() {
        // Bell notification
        let _ = stdout.write_all(b"\x07");
    }
    for damage in damage_events {
        let Ok(hp) = hps.get_mut(entities, damage.target) else { return };

        *hp -= damage.amount;

        if let Ok(timer) = damaged_timer.get_mut(entities, damage.target) {
            timer.reset();
        }
        if *hp <= 0 {
            kill_events.push(Kill { target: damage.target });
        }
    }
}
