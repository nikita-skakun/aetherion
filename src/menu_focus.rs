use bevy::prelude::Resource;

#[derive(Resource)]
pub struct CursorLockState(pub bool);
