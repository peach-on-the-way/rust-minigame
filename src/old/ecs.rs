use std::{
    any::TypeId,
    cell::UnsafeCell,
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntityGeneration(u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entity {
    index: usize,
    generation: EntityGeneration,
}

impl Entity {
    pub const fn index(&self) -> usize {
        self.index
    }

    pub const fn generation(&self) -> EntityGeneration {
        self.generation
    }
}

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}v{}", self.index, self.generation.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComponentAchetype(u128);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QueryAchetype {
    include: u128,
    exclude: u128,
    write: u128,
    read: u128,
    optional: u128,
}

#[derive(Debug)]
pub struct World<Table> {
    unused: HashSet<usize>,
    entities_achetype: Vec<ComponentAchetype>,
    generations: Vec<EntityGeneration>,
    components: HashMap<TypeId, ComponentAchetype>,
    table: Table,
}

impl<Table: AllComponents> Default for World<Table> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Table> World<Table> {
    pub const fn as_unsafe_world_cell(&mut self) -> UnsafeWorld<'_, Table> {
        UnsafeWorld(UnsafeCell::from_mut(self))
    }
}

impl<Table: AllComponents> World<Table> {
    pub fn new() -> World<Table> {
        let components = Table::components();
        assert!(
            components.len() < 127,
            "Currently only supports 127 components"
        );
        World {
            unused: Default::default(),
            generations: Default::default(),
            table: Default::default(),
            entities_achetype: Default::default(),
            components: {
                let mut map = HashMap::default();
                let mut n = 1u128;
                for c in Table::components() {
                    map.insert(c, ComponentAchetype(n));
                    n <<= 1;
                }
                map
            },
        }
    }

    pub fn spawn_empty(&mut self) -> Entity {
        if self.unused.is_empty() {
            self.generations.push(EntityGeneration(0));
            self.entities_achetype.push(ComponentAchetype(0));
            Entity {
                index: self.generations.len() - 1,
                generation: EntityGeneration(0),
            }
        } else {
            let index = *self
                .unused
                .iter()
                .next()
                .expect("unused  is checked for data");
            self.unused.remove(&index);
            let generation = *self
                .generations
                .get(index)
                .expect("the generation already exists");
            Entity { index, generation }
        }
    }

    pub fn despawn(&mut self, entity: Entity) -> Result<(), DespawnError> {
        let generation = self
            .generations
            .get_mut(entity.index)
            .expect("generation exists");
        if *generation != entity.generation {
            return Err(DespawnError::EntityDoesNotExists(entity));
        }
        generation.0 += 1;
        self.table.remove_all(entity);
        Ok(())
    }

    pub fn entity_exists(&self, entity: Entity) -> bool {
        self.generations
            .get(entity.index)
            .is_some_and(|g| entity.generation == *g)
    }

    pub fn get<C>(&self, entity: Entity) -> Result<&C, QueryEntityError>
    where
        Table: Components<C>,
        C: Component,
    {
        if !self.entity_exists(entity) {
            return Err(QueryEntityError::EntityDoesNotExists(entity));
        }
        self.table
            .get(entity.index)
            .ok_or(QueryEntityError::EntityDoesNotHaveComponent(
                entity,
                std::any::type_name::<C>(),
            ))
    }

    pub fn get_mut<C>(&mut self, entity: Entity) -> Result<&mut C, QueryEntityError>
    where
        Table: Components<C>,
        C: Component,
    {
        if !self.entity_exists(entity) {
            return Err(QueryEntityError::EntityDoesNotExists(entity));
        }
        self.table
            .get_mut(entity.index)
            .ok_or(QueryEntityError::EntityDoesNotHaveComponent(
                entity,
                std::any::type_name::<C>(),
            ))
    }

    pub fn query<Q: QueryParam<Table>>(
        &mut self,
    ) -> impl Iterator<Item = <Q::AsRef<'_> as QueryParam<Table>>::Output<'_>> {
        let world = self.as_unsafe_world_cell().0;
        let achetype = unsafe { Q::AsRef::achetype(UnsafeWorld(world)) };
        unsafe { &*world.get() }
            .entities_achetype
            .iter()
            .enumerate()
            .filter(move |(i, a)| {
                !unsafe { &*world.get() }.unused.contains(i) && (a.0 & achetype.include) > 0
            })
            .map(|(i, _)| {
                let g = unsafe { &*world.get() }
                    .generations
                    .get(i)
                    .expect("generation should already exists");
                unsafe {
                    Q::AsRef::get(
                        UnsafeWorld(world),
                        Entity {
                            index: i,
                            generation: *g,
                        },
                    )
                }
            })
    }

    pub fn query_mut<Q: QueryParam<Table>>(&mut self) -> WorldQueryIterMut<Table, Q> {
        WorldQueryIterMut {
            achetype: unsafe { Q::achetype(self.as_unsafe_world_cell()) },
            current_idx: 0,
            _marker: PhantomData,
        }
    }

    pub fn insert<C: Component>(
        &mut self,
        entity: Entity,
        component: C,
    ) -> Result<(), InsertEntityError>
    where
        Table: Components<C>,
    {
        if !self.entity_exists(entity) {
            return Err(InsertEntityError::EntityDoesNotExists(entity));
        }
        for _ in self.table.len()..(entity.index + 1) {
            self.table.insert(None);
        }
        let entity_achetype = self
            .entities_achetype
            .get_mut(entity.index)
            .expect("achetype for this entity should already exists");
        let component_achetype = self
            .components
            .get(&TypeId::of::<C>())
            .expect("component achetype should already exists");
        entity_achetype.0 |= component_achetype.0;
        self.table.insert_at(entity.index, Some(component));
        Ok(())
    }
}

pub struct UnsafeWorld<'a, Table>(&'a UnsafeCell<World<Table>>);
impl<'a, Table> UnsafeWorld<'a, Table> {
    pub unsafe fn as_mut(&self) -> &'a mut World<Table> {
        unsafe { &mut *self.0.get() }
    }

    pub unsafe fn as_ref(&self) -> &'a World<Table> {
        unsafe { &*self.0.get() }
    }

    pub fn into_inner(self) -> &'a UnsafeCell<World<Table>> {
        self.0
    }
}

impl<Table> Clone for UnsafeWorld<'_, Table> {
    fn clone(&self) -> Self {
        UnsafeWorld(self.0)
    }
}
impl<Table> Copy for UnsafeWorld<'_, Table> {}

pub trait Component: 'static {}

pub unsafe trait QueryParam<Table: 'static> {
    type Output<'a>;
    type AsRef<'a>: QueryParam<Table>;
    unsafe fn achetype(world: UnsafeWorld<'_, Table>) -> QueryAchetype;
    unsafe fn get<'a>(world: UnsafeWorld<'a, Table>, entity: Entity) -> Self::Output<'a>;
}

unsafe impl<Table> QueryParam<Table> for Entity
where
    Table: 'static,
{
    type Output<'a> = Entity;
    type AsRef<'a> = Entity;

    unsafe fn achetype(_world: UnsafeWorld<'_, Table>) -> QueryAchetype {
        QueryAchetype {
            include: 0,
            exclude: 0,
            write: 0,
            read: 0,
            optional: 0,
        }
    }

    unsafe fn get<'a>(_world: UnsafeWorld<'a, Table>, entity: Entity) -> Self::Output<'a> {
        entity
    }
}

unsafe impl<Table, C> QueryParam<Table> for &C
where
    Table: Components<C> + 'static,
    C: Component,
{
    type Output<'a> = &'a C;
    type AsRef<'a> = &'a C;

    unsafe fn achetype(world: UnsafeWorld<'_, Table>) -> QueryAchetype {
        let world = unsafe { world.as_ref() };
        let a = *world
            .components
            .get(&std::any::TypeId::of::<C>())
            .expect("achetype to be intilialzed");
        QueryAchetype {
            include: a.0,
            exclude: 0,
            write: 0,
            read: a.0,
            optional: 0,
        }
    }

    unsafe fn get<'a>(world: UnsafeWorld<'a, Table>, entity: Entity) -> Self::Output<'a> {
        let world = unsafe { world.as_ref() };
        world
            .table
            .get(entity.index)
            .expect("component already filtered by achetype")
    }
}

unsafe impl<Table, C> QueryParam<Table> for &mut C
where
    Table: Components<C> + 'static,
    C: Component,
{
    type Output<'a> = &'a mut C;
    type AsRef<'a> = &'a C;

    unsafe fn achetype(world: UnsafeWorld<'_, Table>) -> QueryAchetype {
        let world = unsafe { world.as_ref() };
        let a = *world
            .components
            .get(&std::any::TypeId::of::<C>())
            .expect("achetype to be intilialzed");
        QueryAchetype {
            include: a.0,
            exclude: 0,
            write: a.0,
            read: a.0,
            optional: 0,
        }
    }

    unsafe fn get<'a>(world: UnsafeWorld<'a, Table>, entity: Entity) -> Self::Output<'a> {
        let world = unsafe { world.as_mut() };
        world
            .table
            .get_mut(entity.index)
            .expect("component already filtered by achetype")
    }
}

unsafe impl<Table, C> QueryParam<Table> for Option<&C>
where
    Table: Components<C> + 'static,
    C: Component,
{
    type Output<'a> = Option<&'a C>;
    type AsRef<'a> = Option<&'a C>;

    unsafe fn achetype(world: UnsafeWorld<'_, Table>) -> QueryAchetype {
        let world = unsafe { world.as_ref() };
        let a = *world
            .components
            .get(&std::any::TypeId::of::<C>())
            .expect("achetype to be intilialzed");
        QueryAchetype {
            include: a.0,
            exclude: 0,
            write: a.0,
            read: 0,
            optional: a.0,
        }
    }

    unsafe fn get<'a>(world: UnsafeWorld<'a, Table>, entity: Entity) -> Self::Output<'a> {
        let world = unsafe { world.as_mut() };
        world.table.get(entity.index)
    }
}

unsafe impl<Table, C> QueryParam<Table> for Option<&mut C>
where
    Table: Components<C> + 'static,
    C: Component,
{
    type Output<'a> = Option<&'a mut C>;
    type AsRef<'a> = Option<&'a C>;

    unsafe fn achetype(world: UnsafeWorld<'_, Table>) -> QueryAchetype {
        let world = unsafe { world.as_ref() };
        let a = *world
            .components
            .get(&std::any::TypeId::of::<C>())
            .expect("achetype to be intilialzed");
        QueryAchetype {
            include: a.0,
            exclude: 0,
            write: a.0,
            read: a.0,
            optional: a.0,
        }
    }

    unsafe fn get<'a>(world: UnsafeWorld<'a, Table>, entity: Entity) -> Self::Output<'a> {
        let world = unsafe { world.as_mut() };
        world.table.get_mut(entity.index)
    }
}

macro_rules! query_param_tuple_impl {
    ($($($t:ident)*,)*) => {
        $(
            unsafe impl<Table, $($t,)*> QueryParam<Table> for ($($t,)*)
            where
                Table: 'static,
                $(
                    $t: QueryParam<Table>,
                )*
            {
                type Output<'a> = ($(
                    <$t as QueryParam<Table>>::Output<'a>,
                )*);
                type AsRef<'a> = ($(
                    <$t as QueryParam<Table>>::AsRef<'a>,
                )*);

                unsafe fn achetype(world: UnsafeWorld<'_, Table>) -> QueryAchetype {
                    let mut achetype = QueryAchetype {
                        include: 0,
                        exclude: 0,
                        read: 0,
                        write: 0,
                        optional: 0,
                    };
                    unsafe {
                        $(
                            let next_achetype = <$t as QueryParam<Table>>::achetype(world);
                            if (next_achetype.include & next_achetype.write)
                                & (achetype.include & (achetype.write | achetype.read))
                                > 0
                            {
                                panic!("Conflicting mutable query param found.");
                            }
                            achetype.include |= next_achetype.include;
                            achetype.read |= next_achetype.read;
                            achetype.write |= next_achetype.write;
                            achetype.optional |= next_achetype.optional;
                        )*
                    }
                    achetype
                }

                unsafe fn get<'a>(
                    world: UnsafeWorld<'a, Table>,
                    entity: Entity,
                ) -> Self::Output<'a> {
                    unsafe {
                        ($(
                            <$t as QueryParam<Table>>::get(world, entity),
                        )*)
                    }
                }
            }
        )*
    };
}

query_param_tuple_impl! {
    A,
    A B,
    A B C,
    A B C D,
    A B C D E,
    A B C D E F,
}

pub struct WorldQueryIterMut<Table, Q> {
    achetype: QueryAchetype,
    current_idx: usize,
    _marker: PhantomData<(Table, Q)>,
}

impl<Table, Q> WorldQueryIterMut<Table, Q>
where
    Table: 'static,
    Q: QueryParam<Table>,
{
    pub fn next<'w>(&mut self, world: &'w mut World<Table>) -> Option<Q::Output<'w>> {
        let WorldQueryIterMut {
            achetype,
            current_idx,
            ..
        } = self;
        while world.unused.contains(current_idx)
            || world.entities_achetype.get(*current_idx)?.0 & achetype.include | achetype.optional
                != achetype.include | achetype.optional
        {
            *current_idx += 1;
        }
        let g = *world
            .generations
            .get(*current_idx)
            .expect("generation already exists");
        let q = unsafe {
            Q::get(
                world.as_unsafe_world_cell(),
                Entity {
                    index: *current_idx,
                    generation: g,
                },
            )
        };
        *current_idx += 1;
        Some(q)
    }
}

#[derive(Debug)]
pub enum InsertEntityError {
    EntityDoesNotExists(Entity),
}
impl std::error::Error for InsertEntityError {}
impl std::fmt::Display for InsertEntityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InsertEntityError::EntityDoesNotExists(e) => write!(f, "Entity {e} does not exists"),
        }
    }
}

#[derive(Debug)]
pub enum DespawnError {
    EntityDoesNotExists(Entity),
}
impl std::error::Error for DespawnError {}
impl std::fmt::Display for DespawnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DespawnError::EntityDoesNotExists(e) => write!(f, "Entity {e} does not exists"),
        }
    }
}

#[derive(Debug)]
pub enum QueryEntityError {
    EntityDoesNotExists(Entity),
    EntityDoesNotHaveComponent(Entity, &'static str),
}
impl std::error::Error for QueryEntityError {}
impl std::fmt::Display for QueryEntityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryEntityError::EntityDoesNotExists(e) => write!(f, "Entity {e} does not exists"),
            QueryEntityError::EntityDoesNotHaveComponent(entity, type_name) => {
                write!(f, "Entity {entity} does not have the component {type_name}",)
            }
        }
    }
}

#[derive(Debug)]
pub struct ComponentNotFound;
impl std::error::Error for ComponentNotFound {}
impl std::fmt::Display for ComponentNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Component not found")
    }
}

pub trait AllComponents: Default + 'static {
    fn components() -> Vec<TypeId>;
    fn reserve_all(&mut self, additional: usize);
    fn remove_all(&mut self, entity: Entity);
}

pub trait Components<C: Component>: Default {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn get(&self, index: usize) -> Option<&C>;
    fn get_mut(&mut self, index: usize) -> Option<&mut C>;
    fn insert_at(&mut self, index: usize, component: Option<C>) -> Option<Option<C>>;
    fn insert(&mut self, component: Option<C>) -> usize;
    fn reserve(&mut self, additional: usize);
    fn as_slice(&self) -> &[Option<C>];
    fn as_mut_slice(&mut self) -> &mut [Option<C>];
}

#[derive(Debug)]
pub struct ComponentsData<C: Component> {
    data: Vec<Option<C>>,
}
impl<C: Component> Default for ComponentsData<C> {
    fn default() -> Self {
        ComponentsData { data: vec![] }
    }
}

impl<C: Component> Components<C> for ComponentsData<C> {
    fn len(&self) -> usize {
        self.data.len()
    }

    fn get(&self, index: usize) -> Option<&C> {
        self.data.get(index).and_then(|c| c.as_ref())
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut C> {
        self.data.get_mut(index).and_then(|c| c.as_mut())
    }

    fn insert_at(&mut self, index: usize, component: Option<C>) -> Option<Option<C>> {
        self.data
            .get_mut(index)
            .map(|entry| std::mem::replace(entry, component))
    }

    fn insert(&mut self, component: Option<C>) -> usize {
        self.data.push(component);
        self.data.len() - 1
    }

    fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    fn as_slice(&self) -> &[Option<C>] {
        self.data.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [Option<C>] {
        self.data.as_mut_slice()
    }
}

#[macro_export]
macro_rules! declare_table {
    (
        $table:ident:
        $($name:ident: $component:ty,)*
    ) => {
        #[derive(Debug, Default)]
        pub struct $table {
            $(
                $name: $crate::ecs::ComponentsData<$component>,
            )*
        }

        $(
            impl $crate::ecs::Components<$component> for $table {
                fn len(&self) -> usize {
                    self.$name.len()
                }

                fn get(&self, index: usize) -> Option<&$component> {
                    self.$name.get(index)
                }

                fn get_mut(&mut self, index: usize) -> Option<&mut $component> {
                    self.$name.get_mut(index)
                }

                fn insert_at(&mut self, index: usize, component: Option<$component>) -> Option<Option<$component>> {
                    self.$name.insert_at(index, component)
                }

                fn insert(&mut self, component: Option<$component>) -> usize {
                    self.$name.insert(component)
                }

                fn reserve(&mut self, additional: usize) {
                    self.$name.reserve(additional)
                }

                fn as_slice(&self) -> &[Option<$component>] {
                    self.$name.as_slice()
                }

                fn as_mut_slice(&mut self) -> &mut [Option<$component>] {
                    self.$name.as_mut_slice()
                }
            }
        )*

        impl $crate::ecs::AllComponents for $table {
            fn components() -> Vec<std::any::TypeId> {
                vec![$(
                    ::std::any::TypeId::of::<$component>(),
                )*]
            }

            fn reserve_all(&mut self, additional: usize) {
                $(
                    self.$name.reserve(additional);
                )*
            }

            fn remove_all(&mut self, entity: $crate::ecs::Entity) {
                $(
                    self.$name.insert_at(entity.index, None);
                )*
            }
        }
        $(
            impl $crate::ecs::Component for $component {}
        )*
    };
}

#[cfg(test)]
mod test {
    use super::*;
    #[derive(Debug)]
    struct MyThing(i32);
    #[derive(Debug)]
    struct MyString(String);
    #[derive(Debug)]
    struct MyGong(u32);

    declare_table! {
        MyComponents:
        my_thing: MyThing,
        my_string: MyString,
        my_gong: MyGong,
    }

    #[test]
    fn basic() {
        let mut world = World::<MyComponents>::new();
        let entity = world.spawn_empty();
        world.insert(entity, MyThing(0)).unwrap();
        let entity = world.spawn_empty();
        world.insert(entity, MyThing(1)).unwrap();
        world.insert(entity, MyString("hi".to_string())).unwrap();
        let entity = world.spawn_empty();
        world.insert(entity, MyThing(2)).unwrap();
        world.insert(entity, MyString("omg".to_string())).unwrap();
        world.insert(entity, MyGong(100)).unwrap();
        for c in world.query::<&MyThing>() {
            println!("{}", c.0);
        }
        for c in world.query::<&MyString>() {
            println!("{}", c.0);
        }
        let mut q = world.query_mut::<(&mut MyThing, &mut MyString, Option<&MyGong>)>();
        while let Some((c, s, d)) = q.next(&mut world) {
            c.0 += 1;
            s.0.push_str("hello");
            match d {
                Some(d) => println!("Gong: {}", d.0),
                None => println!("No gone"),
            }
        }

        for c in world.query::<&MyThing>() {
            println!("{}", c.0);
        }
    }
}
