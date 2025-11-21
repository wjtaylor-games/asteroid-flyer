use godot::prelude::*;
use godot::classes::RigidBody3D;
use godot::classes::IRigidBody3D;
use godot::classes::Input;
use godot::builtin::Vector3;


#[derive(GodotClass)]
#[class(base=RigidBody3D)]
struct SpaceShip {
    #[export]
    torque_mag: f32,
    #[export]
    thrust_mag: f32,
    base: Base<RigidBody3D>
}


#[godot_api]
impl IRigidBody3D for SpaceShip {
    fn init(base: Base<RigidBody3D>) -> Self {
        godot_print!("Spaceship!!");

        Self {
            torque_mag: 1.00,
            thrust_mag: 200.0,
            base,
        }
    }

    fn physics_process(&mut self, _delta: f64) {

        let input = Input::singleton();

        let basis = self.base().get_basis();

        let thrust_mult = (input.is_action_pressed("boost") as i32 -
            input.is_action_pressed("retro") as i32) as f32;
        // let thrust_mult = input.is_action_pressed("boost") as i32 as f32;
        let thrust = basis * (Vector3::UP * thrust_mult * self.thrust_mag);

        self.base_mut().apply_force(thrust);

        let pitch_mult = (input.is_action_pressed("pitch_backward") as i32 -
            input.is_action_pressed("pitch_forward") as i32) as f32;

        let yaw_mult = (input.is_action_pressed("yaw_left") as i32 -
            input.is_action_pressed("yaw_right") as i32) as f32;

        let roll_mult = (input.is_action_pressed("roll_right") as i32 -
            input.is_action_pressed("roll_left") as i32) as f32;

        let torque = Vector3::new(pitch_mult, roll_mult, yaw_mult).normalized_or_zero()
            * self.torque_mag;
        let torque_local = basis * torque;

        self.base_mut().apply_torque(torque_local);
    }
}
