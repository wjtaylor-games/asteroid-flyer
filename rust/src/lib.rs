use godot::prelude::*;

struct AsteroidsExtension;

#[gdextension]
unsafe impl ExtensionLibrary for AsteroidsExtension {}

mod spaceship;
mod scalable_rigidbody;
mod asteroid_spawner;

