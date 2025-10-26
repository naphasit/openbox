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

    //# ===== Entity Management =====
    pub fn spawn(&mut self, components: Vec<Box<dyn Any>>) -> Uuid {
        //$ --- Entity Uuid ---
        let uuid = Uuid::new_v4();
        self.entities.push(uuid);

        //$ --- Components ---
        for component in components {
            let type_id = (*component).type_id();
            let entry = self.components.entry(type_id).or_default();

            entry.insert(uuid, component);
        }

        uuid
    }
    pub fn delete(&mut self, uuid: Uuid) {
        //$ --- Delete Entity ---
        for (_, component) in &mut self.components {
            component.remove(&uuid);
        }
    }
}
