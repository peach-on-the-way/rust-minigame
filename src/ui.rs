//! Handles UI

use crate::prelude::*;

/// Visualize the wall at the top and bottom
pub fn visualize_arena_wall_system(stdout: &mut StdoutLock, arena: &Vec2i32, camera: Entity, entities: &Entities, positions: &Components<Vec2i32>) {
    let camera_pos = *positions.get(entities, camera).unwrap();
    let terminal_size = terminal::size().expect("Terminal size");
    let terminal_size = (terminal_size.0 as i32, terminal_size.1 as i32);
    let terminal_middle = (terminal_size.0 / 2, terminal_size.1 / 2);

    let y = -arena.1 - 1;
    let terminal_pos = (terminal_middle.0 + -arena.0 - camera_pos.0, terminal_middle.1 + y - camera_pos.1);
    let char = '━';
    let mut line = String::new();
    for _ in 0..(arena.0 * 2 + 1) {
        line.push(char)
    }
    if pos_in_size(terminal_pos, terminal_size) {
        queue!(stdout, cursor::MoveTo(terminal_pos.0 as u16, terminal_pos.1 as u16), style::Print(line)).unwrap();
    }
    let y = arena.1 + 1;
    let terminal_pos = (terminal_middle.0 + -arena.0 - camera_pos.0, terminal_middle.1 + y - camera_pos.1);
    let char = '━';
    let mut line = String::new();
    for _ in 0..(arena.0 * 2 + 1) {
        line.push(char)
    }
    if pos_in_size(terminal_pos, terminal_size) {
        let _ = queue!(stdout, cursor::MoveTo(terminal_pos.0 as u16, terminal_pos.1 as u16), style::Print(line));
    }
}

/// Display stats about the game and player
pub fn hud_system(stdout: &mut StdoutLock, arena: &Vec2i32, score: &i32, player: &Player, entities: &Entities, hps: &Components<Health>, max_hps: &Components<Health>) {
    let terminal_size = terminal::size().expect("Terminal size");
    let terminal_size = (terminal_size.0 as i32, terminal_size.1 as i32);
    let terminal_middle = (terminal_size.0 / 2, terminal_size.1 / 2);

    let hp = hps.get(entities, player.id).unwrap();
    let max_hp = max_hps.get(entities, player.id).unwrap();
    let weapon = &player.primary_weapon;

    let terminal_pos = (terminal_middle.0 - arena.0, terminal_middle.1 + arena.1 + 2);
    if !pos_in_size(terminal_pos, terminal_size) {
        return;
    }
    let _ = queue!(stdout, cursor::MoveTo(terminal_pos.0 as u16, terminal_pos.1 as u16), style::Print(format!("Health: {hp:>3}/{max_hp:<3}")));
    let _ = queue!(stdout, cursor::MoveTo(terminal_pos.0 as u16, terminal_pos.1 as u16 + 1), style::Print(format!("Weapon: {weapon}")));
    let _ = queue!(stdout, cursor::MoveTo(terminal_pos.0 as u16, terminal_pos.1 as u16 + 2), style::Print(format!("Score : {score}")));
}

/// Show end screen after the player died
pub fn display_end_screen_system(stdout: &mut StdoutLock, score: &i32) {
    let terminal_size = terminal::size().expect("Terminal size");
    let terminal_size = (terminal_size.0 as i32, terminal_size.1 as i32);
    let terminal_middle = (terminal_size.0 / 2, terminal_size.1 / 2);
    let top_text = "You died!";
    let bottom_text = format!("Score: {score}");
    let top_text_pos = (terminal_middle.0 - top_text.len() as i32 / 2, terminal_middle.1);
    let bottom_text_pos = (terminal_middle.0 - bottom_text.len() as i32 / 2, terminal_middle.1 + 1);
    let _ = queue!(stdout, cursor::MoveTo(top_text_pos.0 as u16, top_text_pos.1 as u16), style::Print(top_text));
    let _ = queue!(stdout, cursor::MoveTo(bottom_text_pos.0 as u16, bottom_text_pos.1 as u16), style::Print(bottom_text));
}
