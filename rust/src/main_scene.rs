use godot::prelude::*;
use godot::classes::{Node, INode, AudioStreamPlayer};
use crate::hud::Hud;
use crate::asteroid_spawner::AsteroidSpawner;
use crate::spaceship::SpaceShip;
use crate::spaceship::ViewMode;


#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct MainScene {

    #[init(node = "Hud")]
    hud: OnReady<Gd<Hud>>,

    #[init(node = "AsteroidSpawner")]
    asteroid_spawner: OnReady<Gd<AsteroidSpawner>>,

    #[init(node = "PlayerSpaceShip")]
    player: OnReady<Gd<SpaceShip>>,

    #[init(node = "MusicPlayer")]
    music_player: OnReady<Gd<AudioStreamPlayer>>,

    base: Base<Node>,
}

#[godot_api]
impl INode for MainScene {
    fn ready(&mut self) {
    }
}

#[godot_api]
impl MainScene {
}
