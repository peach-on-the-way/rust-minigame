//! Miscellaneous tools

use crate::prelude::*;

/// Convert position within arena to the collider grid
pub fn arena_collider_pos(arena: &Vec2i32, pos: Vec2i32) -> Vec2usize {
    ((arena.0 + pos.0) as usize, (arena.1 + pos.1) as usize)
}

/// Check if a position is within a rectangle from (0,0) to (rect.0, rect.1)
pub fn pos_in_size(pos: Vec2i32, rect: Vec2i32) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < rect.0 && pos.1 < rect.1
}
