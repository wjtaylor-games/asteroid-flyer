use godot::prelude::*;
use godot::classes::INode;
use godot::builtin::Vector3;
use crate::spaceship;
use crate::scalable_rigidbody::ScalableRigidBody3D;
use std::f32::consts::TAU;

use rand::prelude::*;

const MAXRANGE: f32 = 500.0;


#[derive(GodotClass)]
#[class(base=Node)]
pub struct AsteroidSpawner {
    #[export]
    max_asteroids: i32,
    asteroid_scene: OnReady<Gd<PackedScene>>,
    player: OnReady<Gd<spaceship::SpaceShip>>,
    asteroid_count: i32,
    base: Base<Node>
}

#[godot_api]
impl INode for AsteroidSpawner {
    fn init(base: Base<Node>) -> Self {
        Self {
            max_asteroids: 40,
            asteroid_scene: OnReady::from_loaded("res://asteroid_1.tscn"),
            player: OnReady::from_node("PlayerSpaceShip"),
            asteroid_count: 0,
            base
        }
    }

    fn physics_process(&mut self, _delta: f32) {
        if self.asteroid_count < self.max_asteroids {
            // Make a new asteroid
            let mut new_asteroid = self.asteroid_scene.instantiate_as::<ScalableRigidBody3D>();

            // Generate random properties
            let mut rng = rand::rng();
            let x_pos: f32 = rng.random_range(-MAXRANGE..MAXRANGE);
            let y_pos: f32 = rng.random_range(-MAXRANGE..MAXRANGE);
            let z_pos: f32 = rng.random_range(-MAXRANGE..MAXRANGE);
            let abs_scale: f32 = rng.random_range(10.0..80.0);
            let rotation = random_rotation(&mut rng);

            // Set the properties
            new_asteroid.set_position(Vector3::new(x_pos, y_pos, z_pos));
            new_asteroid.set_rotation(rotation);
            new_asteroid.bind_mut().set_children_scale(
                Vector3::new(abs_scale, abs_scale, abs_scale));

            // Add the new asteroid as a child
            self.base_mut().add_child(&new_asteroid);
            // Increment the count
            self.asteroid_count += 1;
        }
    }
}

fn random_rotation(rng: &mut ThreadRng) -> Vector3 {
    Vector3::new(
        rng.random_range(0.0..TAU),
        rng.random_range(0.0..TAU),
        rng.random_range(0.0..TAU),
    )
}


