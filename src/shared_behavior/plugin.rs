use bevy::prelude::*;

use super::components::{GameCamera, UiCamera};

pub struct SharedBehaviorPlugin;

impl Plugin for SharedBehaviorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, setup_system);
    }
}
fn setup_system(mut commands: Commands) {
    let camera = OrthographicCameraBundle::new_2d();
    let camera_ui = UiCameraBundle::default();

    commands.spawn_bundle(camera).insert(GameCamera);
    commands.spawn_bundle(camera_ui).insert(UiCamera);
}
