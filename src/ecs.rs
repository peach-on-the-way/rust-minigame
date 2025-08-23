#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Entity {
    index: usize,
    generation: u32,
}

#[derive(Default)]
pub struct Entities {
    unused: Vec<usize>,
    generations: Vec<u32>,
    achetype: Vec<bool>,
}

impl Entities {
    pub fn spawn(&mut self) -> Entity {
        if self.unused.is_empty() {
            self.generations.push(0);
            self.achetype.push(true);
            Entity { index: self.generations.len() - 1, generation: 0 }
        } else {
            let index = self.unused.pop().expect("unused is checked for data");
            let generation = *self.generations.get(index).expect("generation for this entity should already exists");
            self.achetype.get_mut(index).expect("achetype for this entity should already exists");
            Entity { index, generation }
        }
    }

    pub fn despawn(&mut self, entity: Entity) -> Result<(), ()> {
        let Some(generation) = self.generations.get_mut(entity.index) else {
            return Err(());
        };
        if *generation != entity.generation {
            return Err(());
        }
        *self.achetype.get_mut(entity.index).expect("Achetype for this entity should already exists") = false;
        *generation += 1;
        Ok(())
    }

    pub fn exists(&self, entity: Entity) -> bool {
        self.generations.get(entity.index).map(|g| *g == entity.generation).unwrap_or(false) && self.achetype.get(entity.index).copied().unwrap_or(false)
    }

    pub fn iter(&self) -> impl Iterator<Item = Entity> {
        self.generations.iter().enumerate().filter_map(|(i, g)| if self.achetype.get(i).copied().unwrap_or(false) { Some(Entity { index: i, generation: *g }) } else { None })
    }
}

pub struct Components<T> {
    data: Vec<Option<T>>,
}

impl<T> Components<T> {
    pub fn insert(&mut self, entities: &Entities, entity: Entity, component: T) -> Result<(), ()> {
        if !entities.exists(entity) {
            return Err(());
        }
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

    pub fn get(&self, entities: &Entities, entity: Entity) -> Result<&T, ()> {
        if !entities.exists(entity) {
            return Err(());
        }
        self.data.get(entity.index).and_then(|d| d.as_ref()).ok_or(())
    }

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

pub type Events<T> = Vec<T>;
