use crate::World;

pub struct Schedule {
    startup: Vec<Box<dyn FnMut(&mut World)>>,
    update: Vec<Box<dyn FnMut(&mut World)>>,
}
