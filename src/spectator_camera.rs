pub use bevy::{input::mouse::MouseMotion, prelude::*};
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    InputManagerBundle,
};

use crate::input::Action;

const CAMERA_MOVE_SPEED: f32 = 5.0;
const CAMERA_VIEW_SPEED: f32 = 3.0;

pub fn move_camera(
    time: Res<Time>,
    mut motion_evr: EventReader<MouseMotion>,
    mut query: Query<(&ActionState<Action>, &mut Transform), With<Camera3d>>,
) {
    let (action_state, mut transform) = query.single_mut();
    let rotation = transform.rotation;
    let mut movement = Vec3::new(0.0, 0.0, 0.0);

    if action_state.pressed(Action::Forward) {
        movement += Vec3::new(0.0, 0.0, -1.0);
    }
    if action_state.pressed(Action::Backward) {
        movement += Vec3::new(0.0, 0.0, 1.0);
    }
    if action_state.pressed(Action::Left) {
        movement += Vec3::new(-1.0, 0.0, 0.0);
    }
    if action_state.pressed(Action::Right) {
        movement += Vec3::new(1.0, 0.0, 0.0);
    }
    if action_state.pressed(Action::Jump) {
        movement += Vec3::new(0.0, 1.0, 0.0);
    }
    if action_state.pressed(Action::Crouch) {
        movement += Vec3::new(0.0, -1.0, 0.0);
    }
    transform.translation +=
        rotation * movement.normalize_or_zero() * time.delta_seconds() * CAMERA_MOVE_SPEED;
    let (mut delta_x, mut delta_y) = (0.0, 0.0);
    for ev in motion_evr.iter() {
        delta_x += ev.delta.x;
        delta_y += ev.delta.y;
    }
    transform.rotate(Quat::from_rotation_y(-delta_x * CAMERA_VIEW_SPEED * 0.001));
    transform.rotate_local(Quat::from_rotation_x(-delta_y * CAMERA_VIEW_SPEED * 0.001));
}

#[derive(Bundle)]
pub struct SpectatorCameraBundle {
    #[bundle]
    pub input_manager: InputManagerBundle<Action>,
    pub camera: Camera3dBundle,
}

impl SpectatorCameraBundle {
    pub fn default_input_map() -> InputMap<Action> {
        use Action::*;
        let mut input_map = InputMap::default();

        //Movement
        input_map.insert(KeyCode::W, Forward);
        input_map.insert(KeyCode::S, Backward);
        input_map.insert(KeyCode::A, Left);
        input_map.insert(KeyCode::D, Right);
        input_map.insert(KeyCode::Space, Jump);
        input_map.insert(KeyCode::LControl, Crouch);

        //Return
        input_map
    }
}
