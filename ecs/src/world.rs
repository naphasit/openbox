use crate::{Components, Entity};
use std::{any::Any, collections::HashMap};
use uuid::Uuid;

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

    pub fn delete(&mut self, entity: Entity) {
        for (_, component) in &mut self.components.0 {
            component.remove(&entity);
        }
    }
}
