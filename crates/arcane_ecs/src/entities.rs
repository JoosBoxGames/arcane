// #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
// pub struct Entity {
//     id: u32,
//     generation: u32,
// }

pub trait Component: 'static + Sized {}

pub struct ECS {
    pub entities: Vec<Component>,
}

impl ECS {
    pub fn create_entity<T: Component>(&mut self, component: T) {
        self.entities.push(component);
    }
}

#[derive(Debug, Clone)]
pub struct Transform {
    pub position: (f32, f32, f32),
    pub rotation: (f32, f32, f32),
}
impl Component for Transform {}

// #[derive(Debug)]
// pub struct Light {
//     intensity: f32,
// }
// impl Component for Light {}


spawn(A, B)
spawn(A, B)
spawn(B, C)

let entities = [{id: 1}]
let components = {
    A: [{ entity_id: 1, value: 10}],
}
