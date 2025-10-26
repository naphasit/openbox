use crate::World;

pub struct Schedule {
    startup: Vec<Box<dyn FnMut(&mut World)>>,
    update: Vec<Box<dyn FnMut(&mut World)>>,
}

impl Schedule {
    pub fn new() -> Self {
        Self {
            startup: Vec::new(),
            update: Vec::new(),
        }
    }
}
