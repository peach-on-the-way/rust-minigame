use crate::prelude::*;

pub fn arena_collider_pos(arena: &Vec2i32, pos: Vec2i32) -> Vec2usize {
    ((arena.0 + pos.0) as usize, (arena.1 + pos.1) as usize)
}

pub fn pos_in_size(pos: Vec2i32, rect: Vec2i32) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < rect.0 && pos.1 < rect.1
}
