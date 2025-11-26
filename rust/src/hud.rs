use godot::prelude::*;
use godot::classes::IControl;
use godot::classes::{Label, Control, AnimationPlayer, Timer};
use crate::spaceship::{ViewMode, SpaceShip};


// Animation names
const NOTIF_FADE_OUT_ANIM : &str = "notification_fade_out";

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct Hud {
    #[init(node = "NotifierLabel")]
    notifier_label: OnReady<Gd<Label>>,
    #[init(node = "NotifierTimer")]
    notifier_timer: OnReady<Gd<Timer>>,
    #[init(node = "AnimationPlayer")]
    animation_player: OnReady<Gd<AnimationPlayer>>,
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
        self.notifier_timer
            .signals()
            .timeout()
            .connect_other(&*self.animation_player, |this| {
                this.set_assigned_animation(NOTIF_FADE_OUT_ANIM);
                this.play();
            });
        self.animation_player
            .signals()
            .animation_finished()
            .connect_other(self, |this, anim_name: StringName| {
                if anim_name == StringName::from(NOTIF_FADE_OUT_ANIM) {
                    this.notifier_label.hide();
                }
            });
        self.notifier_label.hide();
    }
}

#[godot_api]
impl Hud {
    pub fn on_view_mode(&mut self, view_mode: ViewMode) {

        // let mut label = self.base().get_node_as::<Label>("NotifierLabel");
        self.notifier_label
            .set_text(format!("View Mode: {:?}", view_mode)
            .as_str());
        self.notifier_label.show();
        self.notifier_timer.start();
        self.animation_player.set_assigned_animation("RESET");
        self.animation_player.play();
    }
}
