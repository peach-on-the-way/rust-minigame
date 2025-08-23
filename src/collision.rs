use crate::prelude::*;

#[derive(Default)]
pub struct ColliderGrid(pub Vec<Vec<Option<Entity>>>);

impl ColliderGrid {
    pub fn insert(&mut self, pos: Vec2usize, new: Option<Entity>) {
        let Some(slot) = self.0.get_mut(pos.0).and_then(|a| a.get_mut(pos.1)) else { return };
        *slot = new
    }

    pub fn remove(&mut self, pos: Vec2usize) {
        let Some(slot) = self.0.get_mut(pos.0).and_then(|a| a.get_mut(pos.1)) else { return };
        *slot = None;
    }

    pub fn get(&self, pos: Vec2usize) -> Option<Entity> {
        self.0.get(pos.0).and_then(|a| a.get(pos.1)).copied().flatten()
    }
}
