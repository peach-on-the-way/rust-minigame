//! The core ECS implementation.

/// The entity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Entity {
    /// The index which this entity resides in.
    index: usize,
    /// Its generation.
    generation: u32,
}

/// The entities index.
#[derive(Default)]
pub struct Entities {
    /// List of entity's index that has been despawned and its index can be used later.
    unused: Vec<usize>,
    /// The generation of each entity.
    ///
    /// If [`Entity::generation`] at its index do not match, that means that the
    /// entity has been despawned and therefore are invalid.
    generations: Vec<u32>,
    /// The achetype of each entity.
    ///
    /// Currently only stores `bool` which tells if the entity at that index is
    /// valid or not.
    /// In the future, the achetype can be extended to stores additional informations,
    /// such as, the components each entity contains for optimization.
    achetype: Vec<bool>,
}

impl Entities {
    /// Create a new [`Entity`]
    pub fn spawn(&mut self) -> Entity {
        // Check if there are any slot that we can reuse
        if self.unused.is_empty() {
            // `unused` is empty, create a new slot.

            // Initialize the generation to 0
            self.generations.push(0);
            // Initialize the achetype to `true` because the entity is valid.
            self.achetype.push(true);
            Entity { index: self.generations.len() - 1, generation: 0 }
        } else {
            // `unused` contains something, reuse the slot.

            let index = self.unused.pop().expect("unused is checked for data");

            // generation does not need to be incremented here because we've
            // already incremented it on despawn.
            let generation = *self.generations.get(index).expect("generation for this entity should already exists");

            // Set the achetype to `true` since we're reusing the slot.
            let achetype = self.achetype.get_mut(index).expect("achetype for this entity should already exists");
            *achetype = true;

            Entity { index, generation }
        }
    }

    /// Despawn an existing [`Entity`]
    ///
    /// Returns [`Err(())`] if entity does not exists or its generation do not match.
    /// In the future, we can use an actual error type for better user experience.
    pub fn despawn(&mut self, entity: Entity) -> Result<(), ()> {
        let Some(generation) = self.generations.get_mut(entity.index) else {
            return Err(());
        };
        if *generation != entity.generation {
            return Err(());
        }

        // The entity is no longer valid; its achetype is setted to `false`.
        *self.achetype.get_mut(entity.index).expect("achetype for this entity should already exists") = false;

        // Increment the generation and many APIs will now error on generation mismatch.
        *generation += 1;

        Ok(())
    }

    /// Check if an [`Entity`] exists.
    pub fn exists(&self, entity: Entity) -> bool {
        self.generations.get(entity.index).map(|g| *g == entity.generation).unwrap_or(false) && self.achetype.get(entity.index).copied().unwrap_or(false)
    }

    /// Iterate over all valid [`Entity`]s.
    pub fn iter(&self) -> impl Iterator<Item = Entity> {
        self.generations.iter().enumerate().filter_map(|(i, g)| if self.achetype.get(i).copied().unwrap_or(false) { Some(Entity { index: i, generation: *g }) } else { None })
    }
}

/// The Components type.
/// Stores components data where each index refers to an Entity at that index.
pub struct Components<T> {
    data: Vec<Option<T>>,
}

impl<T> Components<T> {
    /// Insert a component to the entity.
    ///
    /// Returns [`Err(())`] if the entity is not valid.
    /// In the future, we can use an actual error type for better user experience.
    pub fn insert(&mut self, entities: &Entities, entity: Entity, component: T) -> Result<(), ()> {
        if !entities.exists(entity) {
            return Err(());
        }
        // Allocate `None` for previous entities if they haven't been allocated yet.
        while self.data.len() < entity.index + 1 {
            self.data.push(None);
        }
        *self.data.get_mut(entity.index).expect("Index should exists") = Some(component);
        Ok(())
    }

    // fn remove(&mut self, entities: &Entities, entity: Entity) -> Result<(), ()> {
    //     if !entities.exists(entity) {
    //         return Err(());
    //     }
    //     if let Some(data) = self.data.get_mut(entity.index) {
    //         *data = None;
    //     }
    //     Ok(())
    // }

    /// Get the component from the entity.
    ///
    /// Returns [`Err(())`] if the entity is not valid or if the entity does not have the component.
    /// In the future, we can use an actual error type for better user experience.
    pub fn get(&self, entities: &Entities, entity: Entity) -> Result<&T, ()> {
        if !entities.exists(entity) {
            return Err(());
        }
        self.data.get(entity.index).and_then(|d| d.as_ref()).ok_or(())
    }

    /// Get the component mutably from the entity.
    ///
    /// Returns [`Err(())`] if the entity is not valid or if the entity does not have the component.
    /// In the future, we can use an actual error type for better user experience.
    pub fn get_mut(&mut self, entities: &Entities, entity: Entity) -> Result<&mut T, ()> {
        if !entities.exists(entity) {
            return Err(());
        }
        self.data.get_mut(entity.index).and_then(|d| d.as_mut()).ok_or(())
    }
}

impl<T> Default for Components<T> {
    fn default() -> Self {
        Components { data: Vec::default() }
    }
}

/// The events buffer is just a [`Vec<T>`] for now.
pub type Events<T> = Vec<T>;
