// TODO: Cache efficiency, parallelism, pages, etc..
use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Entity {
    index: u32,
    generation: u32,
}

pub struct EntitySparseSet {
    locations: Vec<Option<(usize, usize, u32)>>,
}

impl EntitySparseSet {
    pub fn new(capacity: usize) -> Self {
        EntitySparseSet {
            locations: vec![None; capacity],
        }
    }

    pub fn insert(&mut self, entity: Entity, archetype_id: usize, row: usize) {
        if (entity.index as usize) < self.locations.len() {
            self.locations[entity.index as usize] = Some((archetype_id, row, entity.generation));
        }
    }

    pub fn get(&self, entity: Entity) -> Option<(usize, usize)> {
        self.locations
            .get(entity.index as usize)
            .and_then(|location| location.as_ref())
            .and_then(|&(arch_id, row, generation)| {
                if generation == entity.generation {
                    Some((arch_id, row))
                } else {
                    None
                }
            })
    }
}

pub struct Archetype {
    id: usize,
    pub component_types: HashSet<TypeId>,
    pub entities: Vec<Entity>,
    components: HashMap<TypeId, Vec<Box<dyn std::any::Any>>>, // Dense columns per component (simplified; Bevy uses untyped Vec<u8>).
}

impl Archetype {
    pub fn new(id: usize, component_types: HashSet<TypeId>) -> Self {
        Archetype {
            id,
            component_types: component_types.clone(),
            entities: Vec::new(),
            components: component_types
                .iter()
                .map(|&tid| (tid, Vec::new()))
                .collect(),
        }
    }

    pub fn add_entity(
        &mut self,
        entity: Entity,
        comp_data: HashMap<TypeId, Box<dyn std::any::Any>>,
    ) {
        self.entities.push(entity);
        for (tid, data) in comp_data {
            if let Some(column) = self.components.get_mut(&tid) {
                column.push(data);
            }
        }
    }

    pub fn get_component<T: 'static>(&self, row: usize) -> Option<&T> {
        self.components
            .get(&TypeId::of::<T>())?
            .get(row)?
            .downcast_ref()
    }
}

// World: Manages archetypes and sparse entity mappings.
pub struct World {
    pub archetypes: Vec<Archetype>,
    pub entity_sparse: EntitySparseSet,
    next_entity_index: u32,
    next_archetype_id: usize,
}

impl World {
    pub fn new(capacity: usize) -> Self {
        World {
            archetypes: Vec::new(),
            entity_sparse: EntitySparseSet::new(capacity),
            next_entity_index: 0,
            next_archetype_id: 0,
        }
    }

    pub fn spawn(&mut self, comp_data: HashMap<TypeId, Box<dyn std::any::Any>>) -> Entity {
        let entity = Entity {
            index: self.next_entity_index,
            generation: 0,
        };
        self.next_entity_index += 1;

        let component_types: HashSet<TypeId> = comp_data.keys().cloned().collect();
        let arch_id = self.get_or_create_archetype(&component_types);

        let archetype = self.archetypes.get_mut(arch_id).unwrap();
        let row = archetype.entities.len();
        archetype.add_entity(entity, comp_data);

        self.entity_sparse.insert(entity, arch_id, row);
        entity
    }

    pub fn get_or_create_archetype(&mut self, types: &HashSet<TypeId>) -> usize {
        for (i, arch) in self.archetypes.iter().enumerate() {
            if arch.component_types == *types {
                return i;
            }
        }
        let id = self.next_archetype_id;
        self.archetypes.push(Archetype::new(id, types.clone()));
        self.next_archetype_id += 1;
        id
    }

    // Example access: Get a component for an entity.
    pub fn get_component<T: 'static>(&self, entity: Entity) -> Option<&T> {
        let (arch_id, row) = self.entity_sparse.get(entity)?;
        let archetype = &self.archetypes[arch_id];
        archetype.get_component(row)
    }
}

// Example components.
#[derive(Debug)]
pub struct Transform {
    pub pos: (f32, f32),
}
#[derive(Debug)]
pub struct Light {
    pub intensity: f32,
}
#[derive(Debug)]
pub struct Mesh {
    pub size: f32,
}
#[derive(Debug)]
pub struct Material {
    pub color: (f32, f32, f32),
}

// pub trait Component: 'static + Sized {}
