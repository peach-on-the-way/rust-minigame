use crate::prelude::*;

pub struct Damage {
    pub target: Entity,
    pub amount: i32,
}

pub struct Kill {
    pub target: Entity,
}

pub fn damage_system(stdout: &mut Stdout, damage_events: &Events<Damage>, kill_events: &mut Events<Kill>, entities: &Entities, hps: &mut Components<Health>, damaged_timer: &mut Components<Timer>) {
    if !damage_events.is_empty() {
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
