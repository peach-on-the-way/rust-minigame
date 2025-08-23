use crate::prelude::*;

#[derive(Default, Clone)]
pub struct Sprite {
    pub char: char,
    pub style: style::ContentStyle,
}

pub fn sprite_system(stdout: &mut StdoutLock, camera: Entity, entities: &Entities, positions: &Components<Vec2i32>, sprites: &Components<Sprite>, damaged_timer: &Components<Timer>, damaged_color: &Components<Color>) {
    let camera_pos = *positions.get(entities, camera).unwrap();
    let terminal_size = terminal::size().expect("Terminal size");
    let terminal_size = (terminal_size.0 as i32, terminal_size.1 as i32);
    let terminal_middle = (terminal_size.0 / 2, terminal_size.1 / 2);
    for id in entities.iter() {
        let Ok(position) = positions.get(entities, id) else { continue };
        let Ok(sprite) = sprites.get(entities, id) else { continue };

        let terminal_pos = (terminal_middle.0 + position.0 - camera_pos.0, terminal_middle.1 + position.1 - camera_pos.1);
        if !pos_in_size(terminal_pos, terminal_size) {
            continue;
        }
        let mut content = sprite.char.stylize();
        *content.style_mut() = sprite.style;
        if let Ok(timer) = damaged_timer.get(entities, id)
            && let Ok(color) = damaged_color.get(entities, id)
            && !timer.finished()
        {
            content.style_mut().foreground_color = Some(*color);
        }
        let _ = queue!(stdout, cursor::MoveTo(terminal_pos.0 as u16, terminal_pos.1 as u16), style::Print(content));
    }
}
