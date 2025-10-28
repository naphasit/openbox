use logger::error;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};
use uuid::Uuid;

pub struct World {
    entities: Vec<Uuid>,
    components: HashMap<TypeId, HashMap<Uuid, Box<dyn Any>>>,
    resource: HashMap<TypeId, Box<dyn Any>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            components: HashMap::new(),
            resource: HashMap::new(),
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

        self.entities.retain(|&e| e != uuid);
    }

    //# ===== Component Management =====
    pub fn get<T: 'static>(&self, uuid: Uuid) -> Option<&T> {
        let type_id = &TypeId::of::<T>();
        let component = self
            .components
            .get(&TypeId::of::<T>())?
            .get(&uuid)?
            .downcast_ref::<T>();

        if component.is_none() {
            error!("Component {:?} missing in {}", type_id, uuid);
        }

        component
    }
    pub fn get_mut<T: 'static>(&mut self, uuid: Uuid) -> Option<&mut T> {
        let type_id = &TypeId::of::<T>();
        let component = self
            .components
            .get_mut(&TypeId::of::<T>())?
            .get_mut(&uuid)?
            .downcast_mut::<T>();

        if component.is_none() {
            error!("Component {:?} missing in {}", type_id, uuid);
        }

        component
    }

    pub fn query<T: 'static>(&self, mut filter: impl FnMut(&T) -> bool) -> Vec<Uuid> {
        let mut result = Vec::new();

        //$ --- Get Components ---
        let type_id = &TypeId::of::<T>();

        if let Some(components) = self.components.get(type_id) {
            //$ --- Get Components ---
            for (uuid, _) in components {
                if let Some(component) = self.get(*uuid) {
                    //$ --- Filter Component ---
                    if filter(component) {
                        result.push(*uuid);
                    }
                }
            }
        } else {
            error!("Missing component {:?}", type_id);
        }

        result
    }

    //# ===== Resource Management =====
    pub fn insert_resource<T: 'static>(&mut self, resource: Option<T>) {
        let type_id = TypeId::of::<T>();
        self.resource.insert(type_id, Box::new(resource));
    }
    pub fn get_resource<T: 'static>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();

        let resource = self.resource.get(&type_id)?.downcast_ref::<T>();

        if resource.is_none() {
            error!("Missing resource {:?}", type_id);
        }

        resource
    }
    pub fn get_resource_mut<T: 'static>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();

        let resource = self.resource.get_mut(&type_id)?.downcast_mut::<T>();

        if resource.is_none() {
            error!("Missing resource {:?}", type_id);
        }

        resource
    }
}
