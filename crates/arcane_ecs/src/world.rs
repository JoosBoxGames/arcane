use std::{any::TypeId, collections::HashMap};

use crate::Archetype;

pub struct World {
    archetypes: HashMap<TypeId, Archetype>,
}

impl World {
    pub fn new() -> World {
        World {
            archetypes: HashMap::new(),
        }
    }
}
