use crate::prelude::*;

#[derive(Clone, Copy)]
pub enum Shape {
    Rectangle { w: i32, h: i32 },
}

#[derive(Clone)]
pub struct DrawInfo {
    pub sprite: Sprite,
    pub shape: Shape,
    pub timer: Timer,
}

#[derive(Clone)]
pub struct Draw {
    pub particle: DrawInfo,
    pub position: Vec2i32,
}

pub fn spawn_particle_system(spawn_events: &Events<Draw>, entities: &mut Entities, positions: &mut Components<Vec2i32>, particles: &mut Components<DrawInfo>) {
    for spawn in spawn_events {
        let id = entities.spawn();
        particles.insert(entities, id, spawn.particle.clone()).unwrap();
        positions.insert(entities, id, spawn.position).unwrap();
    }
}

pub fn particle_system(stdout: &mut StdoutLock, delta: Duration, camera: Entity, entities: &mut Entities, positions: &Components<Vec2i32>, particles: &mut Components<DrawInfo>) {
    let camera_pos = *positions.get(entities, camera).unwrap();
    let terminal_size = terminal::size().expect("Terminal size");
    let terminal_size = (terminal_size.0 as i32, terminal_size.1 as i32);
    let terminal_middle = (terminal_size.0 / 2, terminal_size.1 / 2);

    let mut to_despawn = vec![];
    for id in entities.iter() {
        let Ok(pos) = positions.get(entities, id) else { continue };
        let Ok(particle) = particles.get_mut(entities, id) else { continue };
        particle.timer.current += delta;
        if particle.timer.finished() {
            to_despawn.push(id);
            continue;
        }
        match particle.shape {
            Shape::Rectangle { w, h } => {
                let mut line = String::new();
                for _ in 0..w {
                    line.push(particle.sprite.char);
                }
                for y in 0..h {
                    let terminal_pos = (terminal_middle.0 + pos.0 - camera_pos.0, terminal_middle.1 + pos.1 - camera_pos.1);
                    if !pos_in_size(terminal_pos, terminal_size) {
                        continue;
                    }
                    let mut content = (&line[..]).stylize();
                    *content.style_mut() = particle.sprite.style;
                    let _ = queue!(stdout, cursor::MoveTo(terminal_pos.0 as u16, y as u16 + terminal_pos.1 as u16), style::PrintStyledContent(content));
                }
            }
        }
    }
    for id in to_despawn {
        let _ = entities.despawn(id);
    }
}
