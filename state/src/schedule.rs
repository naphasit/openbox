use crate::World;
use macroquad::window::next_frame;

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

    //# ===== Run System =====
    pub async fn run(&mut self, world: &mut World) {
        //$ --- Startup ---
        for sys in self.startup.iter_mut() {
            sys(world);
        }

        //$ --- Update ---
        loop {
            for sys in self.update.iter_mut() {
                sys(world);
            }
            next_frame().await;
        }
    }

    //# ===== Add System =====
    pub fn add_startup<T: FnMut(&mut World) + 'static>(&mut self, sys: T) -> &mut Self {
        let boxed_sys = Box::new(sys);
        self.startup.push(boxed_sys);
        self
    }
    pub fn add_update<T: FnMut(&mut World) + 'static>(&mut self, sys: T) -> &mut Self {
        let boxed_sys = Box::new(sys);
        self.update.push(boxed_sys);
        self
    }
}
