use crate::World;

pub struct Systems {
    startup: Vec<Box<dyn FnMut(&mut World)>>,
    update: Vec<Box<dyn FnMut(&mut World)>>,
}

impl Systems {
    pub fn new() -> Self {
        Self {
            startup: Vec::new(),
            update: Vec::new(),
        }
    }

    pub fn run(&mut self, world: &mut World) {
        //* ===== Startup =====
        for sys in self.startup.iter_mut() {
            sys(world);
        }

        //* ===== Update =====
        loop {
            for sys in self.update.iter_mut() {
                sys(world);
            }
        }
    }

    pub fn add<S>(&mut self, sys_type: SystemType, sys: S) -> &Self
    where
        S: FnMut(&mut World) + 'static,
    {
        let boxed_sys: Box<dyn FnMut(&mut World)> = Box::new(sys);
        match sys_type {
            SystemType::Startup => self.startup.push(boxed_sys),
            SystemType::Update => self.update.push(boxed_sys),
        }
        self
    }
}

pub enum SystemType {
    Startup,
    Update,
}
