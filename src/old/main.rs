use rust_minigame::ecs_lite::{self, Entity};

#[derive(Default)]
struct World {
    entities_info: ecs_lite::EntitiesInfo,
    components_info: ecs_lite::ComponentsInfo,
}

impl World {
    pub fn spawn(&mut self) -> Entity {
        self.entities_info.spawn()
    }

    pub fn despawn(&mut self, entity: Entity) -> Result<(), ecs_lite::DespawnError> {
        self.entities_info.despawn(entity)
    }

    pub fn add_component<C: 'static>(&mut self, entity: Entity) {
        let result = self
            .entities_info
            .add_component::<C>(entity, &self.components_info);
        assert!(result)
    }

    pub fn remove_component<C: 'static>(&mut self, entity: Entity) {
        let result = self
            .entities_info
            .remove_component::<C>(entity, &self.components_info);
        assert!(result)
    }
}

fn main() {
    let mut world = World::default();
}
