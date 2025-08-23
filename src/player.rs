use crate::prelude::*;

pub struct Player {
    pub id: Entity,
    pub primary_weapon: Weapon,
}

pub enum Weapon {
    Stick,
}

impl Weapon {
    pub fn base_delay(&self) -> Duration {
        match self {
            Weapon::Stick => Duration::from_millis(700),
        }
    }

    pub fn base_damage(&self) -> i32 {
        match self {
            Weapon::Stick => 4,
        }
    }
}

impl std::fmt::Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Weapon::Stick => write!(f, "Stick"),
        }
    }
}

pub fn player_movement_system(move_timer: &mut Timer, arena: &Vec2i32, player_id: Entity, inputs: &Inputs, collider_grid: &mut ColliderGrid, entities: &mut Entities, positions: &mut Components<Vec2i32>) {
    let mut moved = false;
    let player_pos = positions.get_mut(entities, player_id).unwrap();
    let mut new_pos = *player_pos;
    if move_timer.finished() && inputs.pressed.contains(&KeyCode::Char('w')) && player_pos.1 > -arena.1 {
        new_pos.1 = player_pos.1.saturating_sub(1);
        moved = true;
    }
    if move_timer.finished() && inputs.pressed.contains(&KeyCode::Char('s')) && player_pos.1 < arena.1 {
        new_pos.1 = player_pos.1.saturating_add(1);
        moved = true;
    }
    if move_timer.finished() && inputs.pressed.contains(&KeyCode::Char('a')) && player_pos.0 > -arena.0 {
        new_pos.0 = player_pos.0.saturating_sub(1);
        moved = true;
    }
    if move_timer.finished() && inputs.pressed.contains(&KeyCode::Char('d')) && player_pos.0 < arena.0 {
        new_pos.0 = player_pos.0.saturating_add(1);
        moved = true;
    }
    if moved {
        move_timer.reset();
    }
    if collider_grid.get(arena_collider_pos(arena, new_pos)).is_none() {
        collider_grid.remove(arena_collider_pos(arena, *player_pos));
        *player_pos = new_pos;
        collider_grid.insert(arena_collider_pos(arena, new_pos), Some(player_id));
    }
}

pub fn player_weapon_system(arena: &Vec2i32, player: &Player, weapon_timer: &mut Timer, collider_grid: &ColliderGrid, draw_events: &mut Events<Draw>, damage_events: &mut Events<Damage>, entities: &Entities, inputs: &Inputs, positions: &Components<Vec2i32>) {
    let player_pos = *positions.get(entities, player.id).unwrap();

    let mut dir = (0, 0);
    if inputs.pressed.contains(&KeyCode::Up) {
        dir.1 -= 1
    }
    if inputs.pressed.contains(&KeyCode::Down) {
        dir.1 += 1
    }
    if inputs.pressed.contains(&KeyCode::Left) {
        dir.0 -= 1
    }
    if inputs.pressed.contains(&KeyCode::Right) {
        dir.0 += 1
    }
    if (dir.0 == 0 && dir.1 == 0) || !weapon_timer.finished() {
        return;
    }
    match player.primary_weapon {
        Weapon::Stick => {
            let attack_mid = (player_pos.0 + dir.0 * 2, player_pos.1 + dir.1 * 2);
            let attack_top_left = (attack_mid.0 - 1, attack_mid.1 - 1);
            draw_events.push(Draw { draw_info: DrawInfo { sprite: Sprite { char: '.', ..Default::default() }, shape: Shape::Rectangle { w: 3, h: 3 }, timer: Timer::new(Duration::from_millis(50)) }, position: attack_top_left });
            for x in 0..3 {
                for y in 0..3 {
                    let found = collider_grid.get(arena_collider_pos(arena, (attack_top_left.0 + x, attack_top_left.1 + y)));
                    if let Some(id) = found
                        && id != player.id
                    {
                        damage_events.push(Damage { target: id, amount: player.primary_weapon.base_damage() });
                    }
                }
            }
        }
    }
    weapon_timer.reset();
}

pub fn player_killed_system(kill_events: &Events<Kill>, player_dead: &mut bool, player: &Player) {
    for kill in kill_events {
        if player.id == kill.target {
            *player_dead = true;
        }
    }
}
