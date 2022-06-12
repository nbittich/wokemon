use bevy::prelude::*;

pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Copy, Clone)]
pub enum Foot {
    Left,
    Right,
}

#[derive(Component)]
pub struct GameCamera;
#[derive(Component)]
pub struct UiCamera;

#[derive(Component, Default)]
pub struct Move {
    pub foot: Option<Foot>,
    pub direction: Option<MoveDirection>,
}

impl Move {
    pub fn toggle_foot(&mut self) {
        self.foot = if let Some(Foot::Right) = self.foot {
            Some(Foot::Left)
        } else if let Some(Foot::Left) = self.foot {
            None
        } else {
            Some(Foot::Right)
        };
    }
}
