use std::{
    any::{Any, TypeId},
    collections::HashMap,
};
use uuid::Uuid;

pub struct World {
    entities: Vec<Uuid>,
    components: HashMap<TypeId, HashMap<Uuid, Box<dyn Any>>>,
}
