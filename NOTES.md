# Arcane

## Entity Component System - Sparse
### Entities
Entities are unique identifiers for objects in the game world.
- More than one instance can exist at any given time.
- Unique IDs representing specific objects in the game world.
- Queried and processed individually or in groups.
- Entities with the same component signature are stored together in archetypes

Think: Empty containers or labels

Ex:
- Windows
- Players
- Light sources

### Components
Components are data structures that describe the properties of an entity.
- Components are added to entities to give them new behaviors.
- Components are stored in a sparse array for efficient access.

Think: Properties or characteristics

Ex:
- Position
- Velocity
- Health
- Mesh
- Material

### Systems
Systems are functions that operate on entities and their components.
- Systems process components and update the state of entities.
- Systems are executed in a specific order to ensure consistency.
- Systems define core game logic.

Think: Game logic, behaviors, or rules.

Ex:
- Movement
- Collision detection
- Rendering
- Input handling


## World storage
### World
The central database storing all entities, components, and resources.

Think: The "universe" containing everything in the game.
Access: Through queries in systems.

### Resources
A resource is a singleton.
- There is only one instance per type.
- Resources store global state that affects the whole application.
- Resources can be accessed by any part of the application via systems that need them.

Ex:
- Ambient light
- Game state
- Render settings
- Time


## Application Structure
### App
The main application container.

Think: The "program" that ties everything together.
Contains: World, systems, plugins, schedules.

### Plugins
Plugins are modular bundles of functionality.

Think: Feature packages that extend the application.

Ex:
- Rendering
- Audio
- Physics
- Windowing (e.g. WindowPlugin might create a window entity)


## Execution & Timing
### Schedules
Schedules define collections of systems that run together.

Think: "When" things happen.
Types: Startup (once), Update (every frame), PostUpdate, etc.

### System Ordering
Control when systems run relative to each other.

Think: Dependencies and execution order.


## Communication
### Events
Events are messages sent between systems.

Think: Notifications, triggers, or signals.

Ex:
- Collision event
- Input event
- Animation event

Details from AI research:
1. Thread Safety: Events are Send + Sync + 'static
2. Zero Overhead: Event reading is just iterating over Vec slices
3. Automatic Cleanup: Double buffering ensures events don't accumulate indefinitely
4. System Ordering: EventWriter systems must run before EventReader systems
5. Multiple Readers: Multiple systems can read the same events independently
6. EventMutator: New in recent versions, allows mutating events in-place

### Queries
Queries are how systems access entity data.

Think: Database queries for game objects.

Ex:
`fn enemy_ai(mut enemies: Query<&mut Transform, (With<Enemy>, Without<Player>)>) {}`


## Bevy-Specific Concepts
### Commands
Commands are deferred operations on the world.

Think: "Do this later" - safe way to modify entities during system execution.

Ex:
`commands.spawn(DefaultLight{})`

### Assets
Assets are shared resources like meshes, textures, sounds.

Think: Media files and data that can be reused.

### Change Detection
Automatically track when data changes.

`
fn update_moved_objects(query: Query<&Transform, Changed<Transform>>) {
    for transform in &query {
        // This only runs for entities whose Transform was modified
    }
}
`

### System Parameters
Systems can access data through various parameter types.

Ex:
`fn update_position(mut position: ResMut<Position>, time: Res<Time>) {}`


## Mental Model Summary

Think of Bevy as a theater production:

•  🎭 Entities = Actors, props, lights, cameras (the "things")
•  📋 Components = Costumes, scripts, positions (properties of things)
•  🎬 Systems = Directors, choreographers, lighting operators (the "doers")
•  🏛️ World = The theater building (contains everything)
•  ⚙️ Resources = Theater settings, temperature, house rules (global state)
•  📱 App = The production company (orchestrates everything)
•  🎪 Plugins = Specialist teams (sound, lighting, etc.)
•  📅 Schedules = Rehearsal schedule, showtimes (when things happen)
•  📬 Events = Cues, signals between crew members (communication)
