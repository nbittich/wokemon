use std::time::Duration;

use bevy::prelude::*;

use crate::{
    constants::GAME_OFFSET,
    shared_behavior::components::{Foot, Move, MoveDirection},
};

// region: --- constants

const PLAYER_ASSET: &str = "player.png";
const PLAYER_ASSET_TILE_SIZE: f32 = 32.;
const PLAYER_ASSET_TILE_COLUMNS: usize = 4;
const PLAYER_ASSET_TILE_ROWS: usize = 4;
const PLAYER_BASE_SPEED: f32 = 150.;
const PLAYER_TIME_STEP: f32 = 1. / 60.;
const PLAYER_ASSET_TILE_SCALE: f32 = 2.;
const PLAYER_MOVEMENT_SPEED: u64 = 105;

const MOVEMENT_LEFT_LEFT: usize = 7;
const MOVEMENT_LEFT_RIGHT: usize = 15;
const MOVEMENT_LEFT_NEUTRAL: usize = 3;

const MOVEMENT_UP_LEFT: usize = 14;
const MOVEMENT_UP_RIGHT: usize = 6;
const MOVEMENT_UP_NEUTRAL: usize = 2;

const MOVEMENT_DOWN_LEFT: usize = 13;
const MOVEMENT_DOWN_RIGHT: usize = 5;
const MOVEMENT_DOWN_NEUTRAL: usize = 1;

const MOVEMENT_RIGHT_LEFT: usize = 12;
const MOVEMENT_RIGHT_RIGHT: usize = 4;
const MOVEMENT_RIGHT_NEUTRAL: usize = 0;

// endregion

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
struct PlayerMovementTimer(Timer);

pub struct PlayerPlugin;

#[derive(Clone, Debug)]
struct PlayerTextureAtlas {
    handle: Handle<TextureAtlas>,
    _len: usize,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_player.after("player_atlas"))
            .add_startup_system_to_stage(
                StartupStage::PreStartup,
                setup_player_atlas.label("player_atlas"),
            )
            .add_system(movement_key_input)
            .add_system(movement_translation_system)
            .add_system(movement_texture_system);
    }
}

fn setup_player_atlas(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    assets_server: Res<AssetServer>,
) {
    let texture_atlas = TextureAtlas::from_grid(
        assets_server.load(PLAYER_ASSET),
        Vec2::splat(PLAYER_ASSET_TILE_SIZE),
        PLAYER_ASSET_TILE_COLUMNS,
        PLAYER_ASSET_TILE_ROWS,
    );

    let len = texture_atlas.len();

    let handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(PlayerTextureAtlas { handle, _len: len });
}

fn setup_player(mut commands: Commands, player_atlas: Res<PlayerTextureAtlas>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: player_atlas.handle.clone(),
            transform: Transform {
                scale: Vec3::splat(PLAYER_ASSET_TILE_SCALE),
                ..default()
            },
            ..default()
        })
        .insert(Player)
        .insert(Move { ..default() })
        .insert(PlayerMovementTimer(Timer::new(
            Duration::from_millis(PLAYER_MOVEMENT_SPEED),
            true,
        )));
}

fn movement_key_input(input: Res<Input<KeyCode>>, mut query: Query<&mut Move, With<Player>>) {
    let mut player_move = query.single_mut();

    if !input.pressed(KeyCode::Right)
        && !input.pressed(KeyCode::Left)
        && !input.pressed(KeyCode::Up)
        && !input.pressed(KeyCode::Down)
    {
        if player_move.direction.is_some() || player_move.foot.is_some() {
            player_move.direction = None;
            player_move.foot = None;
        }
    } else if input.pressed(KeyCode::Right) {
        player_move.direction = Some(MoveDirection::Right);
    } else if input.pressed(KeyCode::Left) {
        player_move.direction = Some(MoveDirection::Left);
    } else if input.pressed(KeyCode::Up) {
        player_move.direction = Some(MoveDirection::Up);
    } else if input.pressed(KeyCode::Down) {
        player_move.direction = Some(MoveDirection::Down);
    }
}

fn movement_texture_system(
    timer: Res<Time>,
    mut query: Query<(&mut PlayerMovementTimer, &mut Move, &mut TextureAtlasSprite)>,
) {
    let (mut player_timer, mut movement, mut alias_sprite) = query.single_mut();

    if let Some(direction) = &movement.direction {
        player_timer.tick(timer.delta());
        let index;
        match direction {
            MoveDirection::Left => {
                if let Some(Foot::Left) = movement.foot {
                    index = MOVEMENT_LEFT_LEFT;
                } else if let Some(Foot::Right) = movement.foot {
                    index = MOVEMENT_LEFT_RIGHT;
                } else {
                    index = MOVEMENT_LEFT_NEUTRAL;
                }
            }
            MoveDirection::Up => {
                if let Some(Foot::Left) = movement.foot {
                    index = MOVEMENT_UP_LEFT;
                } else if let Some(Foot::Right) = movement.foot {
                    index = MOVEMENT_UP_RIGHT;
                } else {
                    index = MOVEMENT_UP_NEUTRAL;
                }
            }
            MoveDirection::Down => {
                if let Some(Foot::Left) = movement.foot {
                    index = MOVEMENT_DOWN_LEFT;
                } else if let Some(Foot::Right) = movement.foot {
                    index = MOVEMENT_DOWN_RIGHT;
                } else {
                    index = MOVEMENT_DOWN_NEUTRAL;
                }
            }
            MoveDirection::Right => {
                if let Some(Foot::Left) = movement.foot {
                    index = MOVEMENT_RIGHT_LEFT;
                } else if let Some(Foot::Right) = movement.foot {
                    index = MOVEMENT_RIGHT_RIGHT;
                } else {
                    index = MOVEMENT_RIGHT_NEUTRAL;
                }
            }
        }

        if player_timer.finished() {
            alias_sprite.index = index;
            movement.toggle_foot();
        }
    } else {
        // doesn't move
        if alias_sprite.index >= PLAYER_ASSET_TILE_COLUMNS {
            alias_sprite.index -= PLAYER_ASSET_TILE_COLUMNS;
        }
    }
}

fn movement_translation_system(
    mut query: Query<(&mut Transform, &Move), With<Player>>,
    windows: Res<Windows>,
) {
    let (mut transform, movement) = query.single_mut();
    let mut translation = transform.translation;
    if let Some(direction) = &movement.direction {
        match direction {
            MoveDirection::Up => {
                translation.y += PLAYER_BASE_SPEED * PLAYER_TIME_STEP;
            }
            MoveDirection::Down => {
                translation.y -= PLAYER_BASE_SPEED * PLAYER_TIME_STEP;
            }
            MoveDirection::Left => {
                translation.x -= PLAYER_BASE_SPEED * PLAYER_TIME_STEP;
            }
            MoveDirection::Right => {
                translation.x += PLAYER_BASE_SPEED * PLAYER_TIME_STEP;
            }
        }
    }
    let window = windows.get_primary().unwrap();

    // TODO make the camera follow the player
    if translation.x.abs() < window.width() / 2. - GAME_OFFSET {
        transform.translation.x = translation.x;
    }
    if translation.y.abs() < window.height() / 2. - GAME_OFFSET {
        transform.translation.y = translation.y;
    }
}
