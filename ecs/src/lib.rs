use std::{
    any::{Any, TypeId},
    collections::HashMap,
};
use uuid::Uuid;

pub type Entity = Uuid;

#[derive(Debug)]
pub struct Components(HashMap<TypeId, HashMap<Entity, Box<dyn Any>>>);

pub struct World {
    pub components: Components,
    pub entities: Vec<Entity>,
}

impl World {
    pub fn new() -> Self {
        Self {
            components: Components(HashMap::new()),
            entities: Vec::new(),
        }
    }

    pub fn spawn(&mut self, components: Vec<Box<dyn Any>>) -> Entity {
        //* ===== Add Entity =====
        let entity = Uuid::new_v4();
        self.entities.push(entity);

        //* ===== Add Components =====
        for c in components {
            let type_id = (*c).type_id();
            let entry = self.components.0.entry(type_id).or_insert(HashMap::new());

            entry.insert(entity, c);
        }

        //* ===== Return =====
        entity
    }
}

impl Components {
    fn get<T: 'static>(&self) -> Option<&HashMap<Entity, Box<dyn Any>>> {
        self.0.get(&TypeId::of::<T>())
    }

    fn get_mut<T: 'static>(&mut self) -> Option<&mut HashMap<Entity, Box<dyn Any>>> {
        self.0.get_mut(&TypeId::of::<T>())
    }

    pub fn get_by_entity<T: 'static>(&self, entity: Entity) -> Option<&T> {
        self.get::<T>()?.get(&entity)?.downcast_ref::<T>()
    }

    pub fn get_mut_by_entity<T: 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        self.get_mut::<T>()?.get_mut(&entity)?.downcast_mut::<T>()
    }
}
