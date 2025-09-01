use std::{any::TypeId, collections::HashMap};

use arcane::prelude::*;

fn main() {
    let mut world = World::new(100);

    // Spawn light (archetype: Transform + Light).
    let mut light_comps: HashMap<TypeId, Box<dyn std::any::Any>> = HashMap::new();
    light_comps.insert(
        TypeId::of::<Transform>(),
        Box::new(Transform { pos: (0.0, 10.0) }) as Box<dyn std::any::Any>,
    );
    light_comps.insert(
        TypeId::of::<Light>(),
        Box::new(Light { intensity: 5.0 }) as Box<dyn std::any::Any>,
    );
    let light = world.spawn(light_comps);

    // Spawn cube1 (archetype: Transform + Mesh + Material).
    let mut cube1_comps: HashMap<TypeId, Box<dyn std::any::Any>> = HashMap::new();
    cube1_comps.insert(
        TypeId::of::<Transform>(),
        Box::new(Transform { pos: (1.0, 0.0) }) as Box<dyn std::any::Any>,
    );
    cube1_comps.insert(
        TypeId::of::<Mesh>(),
        Box::new(Mesh { size: 1.0 }) as Box<dyn std::any::Any>,
    );
    cube1_comps.insert(
        TypeId::of::<Material>(),
        Box::new(Material {
            color: (0.5, 0.5, 0.5),
        }) as Box<dyn std::any::Any>,
    );
    let cube1 = world.spawn(cube1_comps);

    // Spawn cube2 (same archetype as cube1).
    let mut cube2_comps: HashMap<TypeId, Box<dyn std::any::Any>> = HashMap::new();
    cube2_comps.insert(
        TypeId::of::<Transform>(),
        Box::new(Transform { pos: (-1.0, 0.0) }) as Box<dyn std::any::Any>,
    );
    cube2_comps.insert(
        TypeId::of::<Mesh>(),
        Box::new(Mesh { size: 1.5 }) as Box<dyn std::any::Any>,
    );
    cube2_comps.insert(
        TypeId::of::<Material>(),
        Box::new(Material {
            color: (0.5, 0.5, 0.5),
        }) as Box<dyn std::any::Any>,
    );
    let _cube2 = world.spawn(cube2_comps);

    // Access example.
    if let Some(transform) = world.get_component::<Transform>(cube1) {
        println!("Cube1 transform: {:?}", transform);
    }

    println!("Archetypes created: {}", world.archetypes.len()); // 2
    if let Some((arch_id, row)) = world.entity_sparse.get(light) {
        println!("Light in archetype {}, row {}", arch_id, row);
    }
}
