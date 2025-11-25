use godot::prelude::*;
use godot::classes::IControl;
use godot::classes::{Label, Control};
use crate::spaceship::{ViewMode, SpaceShip};


#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct Hud {
    #[init(node = "NotifierLabel")]
    notifier_label: OnReady<Gd<Label>>,
    base: Base<Control>,
}
#[godot_api]
impl IControl for Hud {
    fn ready(&mut self) {
        self.base()
            .get_node_as::<SpaceShip>("../PlayerSpaceShip")
            .signals()
            .view_mode_changed()
            .connect_other(&self.to_gd(), Self::on_view_mode);
    }
}

#[godot_api]
impl Hud {
    pub fn on_view_mode(&mut self, view_mode: ViewMode) {

        // let mut label = self.base().get_node_as::<Label>("NotifierLabel");
        self.notifier_label
            .set_text(format!("View Mode: {:?}", view_mode)
            .as_str());
    }
}
