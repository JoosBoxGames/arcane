use arcane::prelude::*;

fn main() {
    let mut ecs = ECS {
        entities: Vec::new(),
    };

    ecs.create_entity(Transform {
        position: (0.0, 0.0, 0.0),
        rotation: (0.0, 0.0, 0.0),
    });

    log_component(&transform);
}

fn log_component<T: Component>(_: &T) {
    println!("Component type: {}", std::any::type_name::<T>());
}
