use arcane::prelude::*;

fn main() {
    let world = World::new();
    let mut commands = Commands::new(world);
    commands.spawn(Transform {});
    commands.spawn(Transform {});
}
