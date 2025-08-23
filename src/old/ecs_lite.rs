use std::{any::TypeId, collections::HashMap};

pub type BitField = u128;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntityGeneration(u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entity {
    index: usize,
    generation: EntityGeneration,
}

#[derive(Debug, Clone, Copy)]
pub struct EntityAchetype {
    valid: bool,
    components: BitField,
}

#[derive(Default, Debug)]
pub struct EntitiesInfo {
    unused: Vec<usize>,
    generations: Vec<EntityGeneration>,
    achetype: Vec<EntityAchetype>,
}

impl EntitiesInfo {
    pub fn spawn(&mut self) -> Entity {
        if self.unused.is_empty() {
            self.generations.push(EntityGeneration(0));
            self.achetype.push(EntityAchetype {
                valid: true,
                components: 0,
            });
            Entity {
                index: self.generations.len() - 1,
                generation: EntityGeneration(0),
            }
        } else {
            let index = self.unused.pop().expect("unused  is checked for data");
            let generation = *self
                .generations
                .get(index)
                .expect("generation for this entity should already exists");
            let achetype = self
                .achetype
                .get_mut(index)
                .expect("achetype for this entity should already exists");
            achetype.valid = true;
            Entity { index, generation }
        }
    }

    pub fn despawn(&mut self, entity: Entity) -> Result<(), DespawnError> {
        let Some(generation) = self.generations.get_mut(entity.index) else {
            return Err(DespawnError::EntityDoesNotExists);
        };
        if *generation != entity.generation {
            return Err(DespawnError::EntityDoesNotExists);
        }
        let achetype = self
            .achetype
            .get_mut(entity.index)
            .expect("achetype should already exists");
        achetype.components = 0;
        achetype.valid = false;
        generation.0 += 1;
        Ok(())
    }

    pub fn add_component<C: 'static>(
        &mut self,
        entity: Entity,
        components_info: &ComponentsInfo,
    ) -> bool {
        let Some(entity_achetype) = self.achetype.get_mut(entity.index) else {
            return false;
        };
        let Some(component_bit) = components_info.get::<C>() else {
            return false;
        };
        entity_achetype.components |= component_bit;
        true
    }

    pub fn remove_component<C: 'static>(
        &mut self,
        entity: Entity,
        components_info: &ComponentsInfo,
    ) -> bool {
        let Some(entity_achetype) = self.achetype.get_mut(entity.index) else {
            return false;
        };
        let Some(component_bit) = components_info.get::<C>() else {
            return false;
        };
        entity_achetype.components ^= component_bit;
        true
    }
}

pub enum DespawnError {
    EntityDoesNotExists,
}

#[derive(Debug)]
pub struct ComponentsInfo {
    next: BitField,
    bitfield_map: HashMap<TypeId, BitField>,
}

impl ComponentsInfo {
    pub fn new() -> ComponentsInfo {
        ComponentsInfo {
            next: 1,
            bitfield_map: HashMap::default(),
        }
    }
    pub fn register<C: 'static>(&mut self) {
        self.bitfield_map.insert(TypeId::of::<C>(), self.next);
        self.next <<= 1;
    }

    pub fn get<C: 'static>(&self) -> Option<BitField> {
        self.bitfield_map.get(&TypeId::of::<C>()).copied()
    }
}

impl Default for ComponentsInfo {
    fn default() -> Self {
        Self::new()
    }
}
