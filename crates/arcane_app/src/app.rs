use crate::plugin::Plugin;

pub struct App {
    plugins: Vec<Box<dyn Plugin>>,
}

impl Default for App {
    fn default() -> Self {
        let app = App::empty();
        app
    }
}

impl App {
    pub fn new() -> App {
        App::default()
    }

    pub fn empty() -> App {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn add_plugin<P: Plugin>(mut self, plugin: P) -> Self {
        println!("Plugin {} added", plugin.name());
        self.plugins.push(Box::new(plugin));
        self
    }

    pub fn run(mut self) {
        println!("App::run() called - starting main loop");

        // Build all plugins - we need to work around the borrowing issue
        // by temporarily taking ownership of the plugins vector
        let mut plugins = std::mem::take(&mut self.plugins);
        for plugin in plugins.iter_mut() {
            plugin.build(&mut self);
        }
        self.plugins = plugins;

        // In a real implementation, this would:
        // 1. Initialize all plugins (done above)
        // 2. Start the event loop (if WinitPlugin is present)
        // 3. Run the game loop
        // 4. Clean up on exit

        // For now, we'll just simulate running
        self.update();

        println!("App::run() completed");
    }

    /// Runs a single update of the app's systems.
    ///
    /// This is useful for testing or headless applications.
    pub fn update(&mut self) {
        // In a real implementation, this would:
        // 1. Process input events
        // 2. Run all scheduled systems
        // 3. Render (if render plugin is present)

        println!("App::update() - running one frame");
    }
}
