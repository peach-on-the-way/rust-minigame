//! Handles collision

use crate::prelude::*;

/// A global resource for indexing grid-wise location for all entities
/// in the arena.
#[derive(Default)]
pub struct ColliderGrid(pub Vec<Vec<Option<Entity>>>);

impl ColliderGrid {
    /// Index an entity
    ///
    /// [`Some(Entity)`] if an entity exists at a location `pos`
    /// [`None`] if there is nothing at the location `pos`
    pub fn insert(&mut self, pos: Vec2usize, new: Option<Entity>) {
        let Some(slot) = self.0.get_mut(pos.0).and_then(|a| a.get_mut(pos.1)) else { return };
        *slot = new
    }

    /// Remove an item at `pos` from the grid
    pub fn remove(&mut self, pos: Vec2usize) {
        let Some(slot) = self.0.get_mut(pos.0).and_then(|a| a.get_mut(pos.1)) else { return };
        *slot = None;
    }

    /// Get entity at `pos`
    ///
    /// Returns [`None`] if there is nothing there.
    pub fn get(&self, pos: Vec2usize) -> Option<Entity> {
        self.0.get(pos.0).and_then(|a| a.get(pos.1)).copied().flatten()
    }
}
