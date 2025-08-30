use arcane_app::{App, Plugin};

#[derive(Default)]
pub struct WinitPlugin;

impl Plugin for WinitPlugin {
    fn build(&self, _app: &mut App) {
        // // Add Winit-specific resources
        // let event_loop = EventLoop::new().unwrap();
        // app.insert_resource(event_loop);

        // // Add systems for window creation and event pumping
        // app.add_systems(Startup, create_window_system);
        // app.add_systems(PreUpdate, pump_events_system);
    }
}

// fn create_window_system(/* params */) {
//     // Use Winit to build the window
//     let window = WindowBuilder::new().build(&event_loop).unwrap();
//     // Insert into Bevy's ECS as a resource or component
// }

// fn pump_events_system(/* params */) {
//     // Run Winit's event loop and forward events to Bevy
// }
