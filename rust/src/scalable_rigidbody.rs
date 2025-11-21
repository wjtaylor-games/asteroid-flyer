use godot::prelude::*;
use godot::classes::RigidBody3D;
use godot::classes::IRigidBody3D;
use godot::builtin::Vector3;
use godot::global::randf_range;

#[derive(GodotClass)]
#[class(base=RigidBody3D)]
struct ScalableRigidBody3D {
    #[export]
    #[var(get = get_children_scale, set = set_children_scale)]
    children_scale: Vector3,
    base: Base<RigidBody3D>
}

#[godot_api]
impl IRigidBody3D for ScalableRigidBody3D {
    fn init(base: Base<RigidBody3D>) -> Self {
        Self {
            children_scale: Vector3::new(1.0, 1.0, 1.0),
            base
        }
    }
}

#[godot_api]
impl ScalableRigidBody3D {
    #[func]
    pub fn get_children_scale(&self) -> Vector3 {
        self.children_scale
    }

    #[func]
    pub fn set_children_scale(&mut self, value: Vector3) {
        godot_print!("Children scale set!");
        self.children_scale = value;

        // Set those children named "CollisionShape3D" and "Mesh"
        // to the new scale.

        let collision_shape = self.base_mut().try_get_node_as::<Node3D>("CollisionShape3D");
        match collision_shape {
            Some(mut collision_shape) => collision_shape.set_scale(value),
            None => (),
        }
        let convex_shape = self.base_mut().try_get_node_as::<Node3D>("ConvexShape");
        match convex_shape {
            Some(mut convex_shape) => convex_shape.set_scale(value),
            None => (),
        }
        let mesh = self.base_mut().try_get_node_as::<Node3D>("Mesh");
        match mesh {
            Some(mut mesh) => mesh.set_scale(value),
            _ => (),
        }
    }

    #[func]
    fn from_random(min_scale: f64, max_scale: f64) -> Gd<Self> {
        // Constructor to create with random scale
        let scale = randf_range(min_scale, max_scale) as f32;
        let vec_scale = Vector3::new(scale, scale, scale);
        Gd::from_init_fn(|base| {
            Self {
                children_scale: vec_scale,
                base,
            }
        })
    }

}
