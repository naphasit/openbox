use crate::Entity;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

#[derive(Debug)]
pub struct Components(pub HashMap<TypeId, HashMap<Entity, Box<dyn Any>>>);

impl Components {
    fn get<T: 'static>(&self) -> Option<&HashMap<Entity, Box<dyn Any>>> {
        self.0.get(&TypeId::of::<T>())
    }

    fn get_mut<T: 'static>(&mut self) -> Option<&mut HashMap<Entity, Box<dyn Any>>> {
        self.0.get_mut(&TypeId::of::<T>())
    }

    pub fn get_all<T: 'static>(&self) -> Vec<(&Entity, Option<&T>)> {
        let mut result = Vec::new();

        if let Some(components) = self.get::<T>() {
            for (entity, data) in components {
                let component_ref = data.downcast_ref::<T>();
                result.push((entity, component_ref));
            }
        }

        result
    }

    pub fn get_all_mut<T: 'static>(&mut self) -> Vec<(&Entity, Option<&mut T>)> {
        let mut result = Vec::new();

        if let Some(components) = self.get_mut::<T>() {
            for (entity, data) in components {
                let component_ref = data.downcast_mut::<T>();
                result.push((entity, component_ref));
            }
        }

        result
    }

    pub fn get_by_entity<T: 'static>(&self, entity: Entity) -> Option<&T> {
        self.get::<T>()?.get(&entity)?.downcast_ref::<T>()
    }

    pub fn get_mut_by_entity<T: 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        self.get_mut::<T>()?.get_mut(&entity)?.downcast_mut::<T>()
    }
}
