use godot::prelude::*;
use godot::classes::{RigidBody3D, IRigidBody3D, Input, InputEvent,
    InputEventMouseMotion, SpringArm3D};
use godot::classes::input::MouseMode;
use std::f32::consts::{TAU, PI};
use godot::global::{wrapf};
use num::clamp;

use godot::builtin::Vector3;


#[derive(GodotConvert, Var, Export)]
#[godot(via = GString)]
#[derive(Clone, Copy, Debug)]
pub enum ViewMode {
    Behind,
    Cockpit,
    Free,
}


#[derive(GodotClass)]
#[class(base=RigidBody3D)]
pub struct SpaceShip {
    #[export]
    torque_mag: f32,
    #[export]
    thrust_mag: f32,
    // zoom distance per mouse wheel tick
    #[export]
    zoom_speed: f32,
    #[export]
    min_zoom: f32,
    #[export]
    max_zoom: f32,
    #[export]
    #[var(set = set_view_mode, get = get_view_mode)]
    view_mode: ViewMode,
    spring_arm_pivot: OnReady<Gd<Node3D>>,
    spring_arm: OnReady<Gd<SpringArm3D>>,
    mouse_sensitivity: f32,
    base: Base<RigidBody3D>,
}


#[godot_api]
impl IRigidBody3D for SpaceShip {
    fn init(base: Base<RigidBody3D>) -> Self {
        godot_print!("Spaceship!!");

        Self {
            torque_mag: 1.00,
            thrust_mag: 200.0,
            zoom_speed: 1.0,
            min_zoom: 2.0,
            max_zoom: 20.0,
            view_mode: ViewMode::Behind,
            spring_arm_pivot: OnReady::from_node("SpringArmPivot"),
            spring_arm: OnReady::from_node("SpringArmPivot/SpringArm3D"),
            mouse_sensitivity: 0.005,
            base,
        }
    }

    fn ready(&mut self) {
        let mut input = Input::singleton();
        input.set_mouse_mode(MouseMode::CAPTURED);
        // Looks redundant, but correctly sets the camera angle.
        self.set_view_mode(self.view_mode);
    }

    fn process(&mut self, _delta: f32) {
        let input = Input::singleton();
        // Toggle the view mode
        if input.is_action_just_pressed("switch_view") {
            self.set_view_mode(match self.view_mode {
                ViewMode::Behind => ViewMode::Cockpit,
                ViewMode::Cockpit => ViewMode::Free,
                ViewMode::Free => ViewMode::Behind,
            })
        }
        match self.view_mode {
            ViewMode::Behind | ViewMode::Free => {
                let curr_length = self.spring_arm.get_length();
                let zoom_dir : f32 = (
                    input.is_action_just_pressed("zoom_out") as i32 -
                    input.is_action_just_pressed("zoom_in") as i32) as f32;
                // godot_print_rich!("zoom_dir: {}", zoom_dir);
                self.spring_arm.set_length(
                    clamp::<f32>(
                        curr_length + zoom_dir * self.zoom_speed,
                        self.min_zoom, self.max_zoom
                    )
                );
            },
            _ => {}
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

    fn input(&mut self, event: Gd<InputEvent>) {
        match event.try_cast::<InputEventMouseMotion>() {
            Ok(e) => {
                match self.view_mode {
                    ViewMode::Free => {
                        let mut rotation = self.spring_arm_pivot.get_rotation();
                        rotation.x = clamp::<f32>(
                            rotation.x - e.get_relative().y * self.mouse_sensitivity,
                            -PI/2.0, PI/2.0);
                        rotation.y = wrapf((rotation.y - e.get_relative().x * self.mouse_sensitivity)
                            as f64, 0.0, TAU as f64) as f32;
                        self.spring_arm_pivot.set_rotation(rotation);
                    },
                    _ => {},
                };
            }
            Err(_) => {}
        }
    }
}

#[godot_api]
impl SpaceShip {

    #[signal]
    pub fn view_mode_changed(new_mode: ViewMode);

    #[func]
    pub fn get_view_mode(&self) -> ViewMode {
        self.view_mode
    }

    #[func]
    pub fn set_view_mode(&mut self, value: ViewMode) {
        self.view_mode = value;
        match value {
            ViewMode::Behind => {
                self.spring_arm_pivot.set_rotation(Vector3::new(PI/2.0, 0.0, 0.0));
                self.spring_arm.set_length(15.0)
            },
            ViewMode::Cockpit => {
                self.spring_arm_pivot.set_rotation(Vector3::new(PI/2.0, 0.0, 0.0));
                self.spring_arm.set_length(-1.0)
            },
            ViewMode::Free => {
                self.spring_arm_pivot.set_rotation(Vector3::new(0.0, (PI/2.0) as f32, 0.0));
                self.spring_arm.set_length(15.0)
            },
        }
        self.signals().view_mode_changed().emit(value);
    }
}
