use std::{
    any::{Any, TypeId},
    collections::HashMap,
};
use uuid::Uuid;

pub struct World {
    entities: Vec<Uuid>,
    components: HashMap<TypeId, HashMap<Uuid, Box<dyn Any>>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            components: HashMap::new(),
        }
    }

    pub fn spawn(&mut self, components: Vec<Box<dyn Any>>) -> Uuid {
        //* ===== Add Entity =====
        let entity = Uuid::new_v4();
        self.entities.push(entity);

        //* ===== Add Components =====
        for c in components {
            let type_id = (*c).type_id();
            let entry = self.components.entry(type_id).or_insert(HashMap::new());

            entry.insert(entity, c);
        }

        //* ===== Return =====
        entity
    }

    pub fn delete(&mut self, entity: Uuid) {
        for (_, component) in &mut self.components {
            component.remove(&entity);
        }
    }
}
