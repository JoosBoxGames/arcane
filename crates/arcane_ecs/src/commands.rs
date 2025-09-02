use std::any::type_name;

use crate::World;

// TODO: Launch - Better name than Commands?
pub struct Commands {
    world: World,
}
impl Commands {
    pub fn new(world: World) -> Self {
        Self { world: world }
    }

    pub fn spawn<T>(&mut self, bundle: T) {
        println!("Creating entity in world: {}", type_name::<T>());
        // self.world.archetypes;
    }
}
