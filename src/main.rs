use bevy::prelude::*;
use wokemon::constants::{GAME_HEIGHT, GAME_WIDTH};
pub use wokemon::player::PlayerPlugin;
pub use wokemon::shared_behavior::SharedBehaviorPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BEIGE))
        .insert_resource(WindowDescriptor {
            width: GAME_WIDTH,
            height: GAME_HEIGHT,
            title: "Wokemon".to_string(),
            resizable: true,
            present_mode: bevy::window::PresentMode::Fifo,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(SharedBehaviorPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
