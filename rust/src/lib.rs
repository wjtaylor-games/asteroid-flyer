use godot::prelude::*;

struct AsteroidsExtension;

#[gdextension]
unsafe impl ExtensionLibrary for AsteroidsExtension {}

mod main_scene;
mod hud;
mod spaceship;
mod scalable_rigidbody;
mod asteroid_spawner;

