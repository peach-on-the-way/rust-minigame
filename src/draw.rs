//! Handles simple terminal drawing.

use crate::prelude::*;

/// Mathematical shapes.
#[derive(Clone, Copy)]
pub enum Shape {
    Rectangle { w: i32, h: i32 },
}

/// A component used for storing drawing information.
#[derive(Clone)]
pub struct DrawInfo {
    pub sprite: Sprite,
    pub shape: Shape,
}

/// An event emitted by any system to draw something.
#[derive(Clone)]
pub struct Draw {
    /// The drawing information.
    pub draw_info: DrawInfo,
    /// The position.
    pub position: Vec2i32,
    /// Time to persist for.
    pub timer: Timer,
}

/// Check for [`Draw`] event and spawn the entity for it.
pub fn spawn_draw_system(spawn_events: &Events<Draw>, entities: &mut Entities, positions: &mut Components<Vec2i32>, draw_infos: &mut Components<DrawInfo>, draw_timer: &mut Components<Timer>) {
    for spawn in spawn_events {
        let id = entities.spawn();
        draw_infos.insert(entities, id, spawn.draw_info.clone()).unwrap();
        positions.insert(entities, id, spawn.position).unwrap();
        draw_timer.insert(entities, id, spawn.timer.clone()).unwrap();
    }
}

/// Draw [`DrawInfo`] to the terminal.
pub fn draw_system(stdout: &mut StdoutLock, camera: Entity, entities: &mut Entities, positions: &Components<Vec2i32>, draw_infos: &mut Components<DrawInfo>, draw_timer: &Components<Timer>) {
    let camera_pos = *positions.get(entities, camera).unwrap();
    let terminal_size = terminal::size().expect("Terminal size");
    let terminal_size = (terminal_size.0 as i32, terminal_size.1 as i32);
    let terminal_middle = (terminal_size.0 / 2, terminal_size.1 / 2);

    // Buffer for entities that needed to be respawn. (Solves mut aliasing issue)
    let mut to_despawn = vec![];
    for id in entities.iter() {
        let Ok(pos) = positions.get(entities, id) else { continue };
        let Ok(draw_info) = draw_infos.get_mut(entities, id) else { continue };
        let Ok(timer) = draw_timer.get(entities, id) else { continue };

        // Despawn instead if the timer is finishes.
        if timer.finished() {
            to_despawn.push(id);
            continue;
        }

        match draw_info.shape {
            Shape::Rectangle { w, h } => {
                // Print line by line instead of character to reduce bottleneck.
                let mut line = String::new();
                for _ in 0..w {
                    line.push(draw_info.sprite.char);
                }
                for y in 0..h {
                    let terminal_pos = (terminal_middle.0 + pos.0 - camera_pos.0, terminal_middle.1 + pos.1 - camera_pos.1);
                    if !pos_in_size(terminal_pos, terminal_size) {
                        continue;
                    }
                    let mut content = (&line[..]).stylize();
                    *content.style_mut() = draw_info.sprite.style;
                    let _ = queue!(stdout, cursor::MoveTo(terminal_pos.0 as u16, y as u16 + terminal_pos.1 as u16), style::PrintStyledContent(content));
                }
            }
        }
    }

    for id in to_despawn {
        let _ = entities.despawn(id);
    }
}
