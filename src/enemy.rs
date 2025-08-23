use crate::prelude::*;

pub fn spawn_enemy_system(arena: &Vec2i32, enemies: &mut HashSet<Entity>, collider_grid: &mut ColliderGrid, entities: &mut Entities, sprites: &mut Components<Sprite>, positions: &mut Components<Vec2i32>, hps: &mut Components<Health>, move_timers: &mut Components<Timer>, damaged_timer: &mut Components<Timer>, damaged_color: &mut Components<Color>) {
    let enemy_id = entities.spawn();
    let mut pos = (rand::random_range(-arena.0..=arena.0), rand::random_range(-arena.1..=arena.1));
    while collider_grid.get(arena_collider_pos(arena, pos)).is_some() {
        pos = (rand::random_range(-arena.0..=arena.0), rand::random_range(-arena.1..=arena.1));
    }
    let special = rand::random_bool(0.1);
    collider_grid.insert(arena_collider_pos(arena, pos), Some(enemy_id));
    if special {
        sprites.insert(entities, enemy_id, Sprite { char: '%', style: style::ContentStyle { foreground_color: Some(Color::AnsiValue(75)), ..Default::default() } }).unwrap();
        move_timers.insert(entities, enemy_id, Timer { current: Duration::ZERO, max: Duration::from_millis(100) }).unwrap();
    } else {
        sprites.insert(entities, enemy_id, Sprite { char: '$', style: style::ContentStyle { foreground_color: Some(Color::AnsiValue(218)), ..Default::default() } }).unwrap();
        move_timers.insert(entities, enemy_id, Timer { current: Duration::ZERO, max: Duration::from_millis(300) }).unwrap();
    }
    positions.insert(entities, enemy_id, pos).unwrap();
    hps.insert(entities, enemy_id, 10).unwrap();
    damaged_timer.insert(entities, enemy_id, Timer::new_ended(Duration::from_millis(200))).unwrap();
    damaged_color.insert(entities, enemy_id, Color::Red).unwrap();
    enemies.insert(enemy_id);
}

pub fn enemy_follow_system(arena: &Vec2i32, player: &Player, enemies: &HashSet<Entity>, collider_grid: &mut ColliderGrid, damage_events: &mut Events<Damage>, entities: &Entities, positions: &mut Components<Vec2i32>, move_timers: &mut Components<Timer>) {
    let player_pos = *positions.get(entities, player.id).unwrap();
    for enemy_id in enemies.iter() {
        let Ok(enemy_pos) = positions.get_mut(entities, *enemy_id) else { continue };
        let Ok(timer) = move_timers.get_mut(entities, *enemy_id) else { continue };
        if timer.current < timer.max {
            continue;
        }
        let mut new_pos = *enemy_pos;
        if player_pos.0 > enemy_pos.0 {
            new_pos.0 += 1;
        } else if player_pos.0 < enemy_pos.0 {
            new_pos.0 -= 1;
        }
        if player_pos.1 > enemy_pos.1 {
            new_pos.1 += 1;
        } else if player_pos.1 < enemy_pos.1 {
            new_pos.1 -= 1;
        }
        if let Some(id) = collider_grid.get(arena_collider_pos(arena, new_pos)) {
            if id == player.id {
                damage_events.push(Damage { target: player.id, amount: 1 });
            } else {
                continue;
            }
        } else {
            collider_grid.remove(arena_collider_pos(arena, *enemy_pos));
            *enemy_pos = new_pos;
            collider_grid.insert(arena_collider_pos(arena, new_pos), Some(*enemy_id));
        }
        timer.current = Duration::ZERO;
    }
}

pub fn enemy_killed_system(arena: &Vec2i32, kill_events: &Events<Kill>, collider_grid: &mut ColliderGrid, score: &mut i32, enemies: &mut HashSet<Entity>, entities: &mut Entities, positions: &Components<Vec2i32>) {
    for dead in kill_events {
        if !enemies.contains(&dead.target) {
            continue;
        }
        let pos = positions.get(entities, dead.target).unwrap();
        entities.despawn(dead.target).unwrap();
        enemies.remove(&dead.target);
        collider_grid.remove(arena_collider_pos(arena, *pos));
        *score += 1;
    }
}
